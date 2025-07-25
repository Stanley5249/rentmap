use std::path::PathBuf;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::types::Json;
use sqlx::{QueryBuilder, SqlitePool};
use tracing::{debug, info};
use url::Url;

use super::WorkspaceError;
use crate::file::make_directory;
use crate::sites::rent591::{RentItem, RentList};
use crate::web::Page;

#[derive(Clone, Debug)]
pub struct Workspace {
    pub root: PathBuf,
    pub pool: SqlitePool,
}

impl Workspace {
    /// Create a new workspace with the given root directory
    pub fn new(root: PathBuf) -> Self {
        let path = root.join("rentmap.sqlite");
        let options = SqliteConnectOptions::new()
            .filename(path)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true)
            .synchronous(SqliteSynchronous::Normal)
            .optimize_on_close(true, None);
        let pool = SqlitePool::connect_lazy_with(options);

        Self { root, pool }
    }

    pub async fn init(&self) -> Result<(), WorkspaceError> {
        make_directory(&self.root)?;
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }

    // List operations

    /// Check if a list exists for the given URL
    pub async fn list_exists(&self, url: &Url) -> Result<bool, WorkspaceError> {
        let exists = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM rent_list WHERE url = ?)")
            .bind(Json(url))
            .fetch_one(&self.pool)
            .await?;

        debug!("check list exists");

        Ok(exists)
    }

    /// Insert a new rent list with item summaries
    pub async fn insert_list(&self, list: &RentList) -> Result<(), WorkspaceError> {
        let mut tx = self.pool.begin().await?;

        let id: i64 = sqlx::query_scalar(
            "INSERT INTO rent_list (url, page_count, item_count) VALUES (?, ?, ?) RETURNING id",
        )
        .bind(&list.url)
        .bind(list.page_count)
        .bind(list.item_count)
        .fetch_one(&mut *tx)
        .await?;

        for summary in list.item_summaries() {
            sqlx::query(
                "INSERT OR REPLACE INTO rent_item_summary (list_id, url, title, price, tags, txts, images) VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(id)
            .bind(&summary.url)
            .bind(&summary.title)
            .bind(&summary.price)
            .bind(&summary.tags)
            .bind(&summary.txts)
            .bind(&summary.images)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        info!("insert list");

        Ok(())
    }

    /// Get the latest list for a URL
    pub async fn select_list(&self, url: &Url) -> Result<Option<RentList>, WorkspaceError> {
        let rent_list = sqlx::query_as("SELECT url, page_count, item_count FROM rent_list WHERE url = ? ORDER BY created_at DESC LIMIT 1")
            .bind(Json(url))
            .fetch_optional(&self.pool)
            .await?;

        debug!("select list");

        Ok(rent_list)
    }

    // Item operations

    /// Check if an item exists for the given URL
    pub async fn item_exists(&self, url: &Url) -> Result<bool, WorkspaceError> {
        let exists = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM rent_item WHERE url = ?)")
            .bind(Json(url))
            .fetch_one(&self.pool)
            .await?;

        debug!("check item exists");

        Ok(exists)
    }

    /// Insert multiple rent items
    pub async fn insert_items<'a, I>(&self, items: I) -> Result<(), WorkspaceError>
    where
        I: IntoIterator<Item = &'a RentItem>,
    {
        let mut tx = self.pool.begin().await?;
        let items: Vec<_> = items.into_iter().collect();

        for item in &items {
            sqlx::query(
"INSERT OR REPLACE INTO rent_item (url, title, labels, patterns, content, phone, album, area, floor, price, address) 
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&item.url)
                .bind(&item.title)
                .bind(&item.labels)
                .bind(&item.patterns)
                .bind(&item.content)
                .bind(&item.phone)
                .bind(&item.album)
                .bind(&item.area)
                .bind(&item.floor)
                .bind(&item.price)
                .bind(&item.address)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        info!(count = items.len(), "insert items");

        Ok(())
    }

    /// Get the latest item for a URL
    pub async fn select_item(&self, url: &Url) -> Result<Option<RentItem>, WorkspaceError> {
        let item = sqlx::query_as("SELECT url, title, labels, patterns, content, phone, album, area, floor, price, address FROM rent_item WHERE url = ?")
            .bind(Json(url))
            .fetch_optional(&self.pool)
            .await?;

        debug!("select item");

        Ok(item)
    }

    // Complex queries

    /// Get item URLs from the latest list, optionally filtered and limited
    pub async fn select_item_urls_with(
        &self,
        list_url: &Url,
        refresh: bool,
        limit: Option<u32>,
    ) -> Result<Vec<Json<Url>>, WorkspaceError> {
        let mut builder =
            QueryBuilder::new("WITH LatestList AS (SELECT id FROM rent_list WHERE url = ");
        builder.push_bind(Json(list_url));
        builder.push(" ORDER BY created_at DESC LIMIT 1) SELECT DISTINCT ris.url FROM rent_item_summary ris JOIN LatestList ll ON ris.list_id = ll.id");

        if !refresh {
            builder.push(" WHERE NOT EXISTS (SELECT 1 FROM rent_item ri WHERE ri.url = ris.url)");
        }

        builder.push(" ORDER BY ris.url");

        if let Some(limit) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(limit);
        }

        let urls = builder.build_query_scalar().fetch_all(&self.pool).await?;

        info!(url_count = urls.len(), refresh, limit, "select item urls");

        Ok(urls)
    }

    /// Get all latest items from a list
    pub async fn select_items(&self, url: &Url) -> Result<Vec<RentItem>, WorkspaceError> {
        let items = sqlx::query_as(
            "
WITH LatestList AS (
    SELECT id FROM rent_list WHERE url = ? ORDER BY created_at DESC LIMIT 1
)
SELECT
    ri.url, ri.title, ri.labels, ri.patterns, ri.content,
    ri.phone, ri.album, ri.area, ri.floor, ri.price, ri.address
FROM rent_item ri
JOIN rent_item_summary ris ON ri.url = ris.url
JOIN LatestList ll ON ris.list_id = ll.id",
        )
        .bind(Json(url))
        .fetch_all(&self.pool)
        .await?;

        info!(count = items.len(), "select items in list");

        Ok(items)
    }

    // Page cache operations

    /// Get cached page HTML by URL
    pub async fn get_cached_page(&self, url: &Url) -> Result<Option<Page>, WorkspaceError> {
        let page = sqlx::query_as("SELECT url, html FROM page_cache WHERE url = ?")
            .bind(Json(url))
            .fetch_optional(&self.pool)
            .await?;

        if page.is_some() {
            debug!("cached page found");
        } else {
            debug!("cached page not found");
        }

        Ok(page)
    }

    /// Cache a page's HTML content
    pub async fn cache_page(&self, page: &Page) -> Result<(), WorkspaceError> {
        sqlx::query("INSERT OR REPLACE INTO page_cache (url, html) VALUES (?, ?)")
            .bind(&page.url)
            .bind(&page.html)
            .execute(&self.pool)
            .await?;

        debug!("cache page");

        Ok(())
    }
}
