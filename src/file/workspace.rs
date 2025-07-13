use std::path::{Path, PathBuf};

use clap::Args;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::types::Json;
use sqlx::{QueryBuilder, SqlitePool};
use tracing::{debug, info};
use url::Url;

use super::{FileError, make_directory, save_html};
use crate::sites::rent591::{RentItem, RentList};
use crate::url::UrlExt;
use crate::web::Page;

#[derive(Debug, Args)]
pub struct WorkspaceArgs {
    /// The root directory of the workspace
    #[arg(long, short, default_value = ".rentmap")]
    pub workspace: PathBuf,
}

impl WorkspaceArgs {
    pub async fn build(self) -> Result<Workspace, FileError> {
        let workspace = Workspace::new(self.workspace);
        workspace.init().await?;
        Ok(workspace)
    }
}

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

    pub async fn init(&self) -> Result<(), FileError> {
        make_directory(&self.root)?;
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }

    #[allow(dead_code)]
    fn html_file_for_read<P>(&self, file_name: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let mut path = self.root.join("html");
        path.push(file_name);
        path
    }

    fn html_file_for_write<P>(&self, file_name: P) -> Result<PathBuf, FileError>
    where
        P: AsRef<Path>,
    {
        let mut path = self.root.join("html");
        make_directory(&path)?;
        path.push(file_name);
        Ok(path)
    }

    pub fn save_page(&self, page: &Page) -> Result<(), FileError> {
        let path = page.url_final.to_path_buf();
        let path = self.html_file_for_write(path)?;
        save_html(&page.html, path)
    }

    // List operations

    /// Check if a list exists for the given URL
    pub async fn list_exists(&self, url: &Url) -> Result<bool, FileError> {
        let exists = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM rent_list WHERE url = ?)")
            .bind(Json(url))
            .fetch_one(&self.pool)
            .await?;

        debug!("check list exists");

        Ok(exists)
    }

    /// Insert a new rent list with item summaries
    pub async fn insert_list(&self, list: &RentList) -> Result<(), FileError> {
        let mut tx = self.pool.begin().await?;

        let id: i64 = sqlx::query_scalar(
            "INSERT INTO rent_list (created_at, url, page_count, item_count) VALUES (?, ?, ?, ?) RETURNING id"
        )
        .bind(list.created_at)
        .bind(&list.url)
        .bind(list.page_count)
        .bind(list.item_count)
        .fetch_one(&mut *tx)
        .await?;

        for summary in list.item_summaries() {
            sqlx::query(
                "INSERT INTO rent_item_summary (list_id, url, title, price, tags, txts, images) VALUES (?, ?, ?, ?, ?, ?, ?)"
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
    pub async fn select_list(&self, url: &Url) -> Result<Option<RentList>, FileError> {
        let rent_list = sqlx::query_as("SELECT id, created_at, url, page_count, item_count FROM rent_list WHERE url = ? ORDER BY created_at DESC LIMIT 1")
            .bind(Json(url))
            .fetch_optional(&self.pool)
            .await?;

        debug!("select list");

        Ok(rent_list)
    }

    // Item operations

    /// Check if an item exists for the given URL
    pub async fn item_exists(&self, url: &Url) -> Result<bool, FileError> {
        let exists = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM rent_item WHERE url = ?)")
            .bind(Json(url))
            .fetch_one(&self.pool)
            .await?;

        debug!("check item exists");

        Ok(exists)
    }

    /// Insert multiple rent items
    pub async fn insert_items<'a, I>(&self, items: I) -> Result<(), FileError>
    where
        I: IntoIterator<Item = &'a RentItem>,
    {
        let mut tx = self.pool.begin().await?;
        let items: Vec<_> = items.into_iter().collect();

        for item in &items {
            sqlx::query(
"INSERT INTO rent_item (created_at, url, title, labels, patterns, content, phone, album, area, floor, price, address) 
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(item.created_at)
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
    pub async fn select_item(&self, url: &Url) -> Result<Option<RentItem>, FileError> {
        let item = sqlx::query_as("SELECT id, created_at, url, title, labels, patterns, content, phone, album, area, floor, price, address FROM rent_item WHERE url = ? ORDER BY created_at DESC LIMIT 1")
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
    ) -> Result<Vec<Json<Url>>, FileError> {
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
    pub async fn select_items(&self, url: &Url) -> Result<Vec<RentItem>, FileError> {
        let items = sqlx::query_as(
            "
WITH LatestList AS (
    SELECT id FROM rent_list WHERE url = ? ORDER BY created_at DESC LIMIT 1
),
RankedItems AS (
    SELECT
        ri.*, ROW_NUMBER() OVER (PARTITION BY ri.url ORDER BY ri.created_at DESC) as rn
    FROM rent_item ri
    JOIN rent_item_summary ris ON ri.url = ris.url
    JOIN LatestList ll ON ris.list_id = ll.id
)
SELECT
    id, created_at, url, title, labels, patterns, content,
    phone, album, area, floor, price, address
FROM RankedItems 
WHERE rn = 1",
        )
        .bind(Json(url))
        .fetch_all(&self.pool)
        .await?;

        info!(count = items.len(), "select items in list");

        Ok(items)
    }
}
