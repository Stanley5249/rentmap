use std::ops::DerefMut;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use clap::Args;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::FileError;
use super::ops::{load_json, make_directory, save_html, save_json};
use crate::file::exists_and_non_empty;
use crate::url::UrlExt;
use crate::web::page::Page;

type UpdateResult<T> = Result<T, T>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimedRecord<T> {
    pub timestamp: DateTime<Utc>,
    pub data: T,
}

pub type TimedRecords<T> = Vec<TimedRecord<T>>;

impl<T> TimedRecord<T> {
    pub fn new(timestamp: DateTime<Utc>, data: T) -> Self {
        Self { timestamp, data }
    }

    pub fn now(data: T) -> Self {
        Self {
            timestamp: Utc::now(),
            data,
        }
    }
}

impl<T> From<T> for TimedRecord<T> {
    fn from(data: T) -> Self {
        Self::now(data)
    }
}

pub trait SortByTimestamp {
    fn sort_by_timestamp(&mut self);
}

impl<T, U> SortByTimestamp for T
where
    T: DerefMut<Target = [TimedRecord<U>]>,
{
    fn sort_by_timestamp(&mut self) {
        self.deref_mut().sort_unstable_by_key(|r| r.timestamp);
    }
}

#[derive(Clone, Debug, Args)]
pub struct Workspace {
    /// The root directory of the workspace
    #[arg(long = "workspace", short = 'w', default_value = ".rentmap")]
    pub root: PathBuf,
}

impl Workspace {
    fn data_file_for_read<P>(&self, file_name: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let mut path = self.root.join("data");
        path.push(file_name);
        path
    }

    fn data_file_for_write<P>(&self, file_name: P) -> Result<PathBuf, FileError>
    where
        P: AsRef<Path>,
    {
        let mut path = self.root.join("data");
        make_directory(&path)?;
        path.push(file_name);
        Ok(path)
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

    pub fn init(&self) -> Result<(), FileError> {
        make_directory(&self.root)
    }

    pub fn save_page(&self, page: &Page) -> Result<(), FileError> {
        let path = page.url_final.to_path_buf();
        let path = self.html_file_for_write(path)?;
        save_html(&page.html, path)
    }

    pub fn save_records<T, P>(
        &self,
        records: &TimedRecords<T>,
        file_name: P,
    ) -> Result<(), FileError>
    where
        T: Serialize,
        P: AsRef<Path>,
    {
        let path = self.data_file_for_write(file_name)?;
        save_json(records, &path)
    }

    pub fn load_records<T, P>(&self, file_name: P) -> Result<TimedRecords<T>, FileError>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        let path = self.data_file_for_read(file_name);
        let mut records = load_records_or_default(&path)?;
        records.sort_by_timestamp();
        Ok(records)
    }

    pub fn add_record<T, P>(
        &self,
        value: TimedRecord<T>,
        file_name: P,
    ) -> Result<TimedRecords<T>, FileError>
    where
        T: Serialize + DeserializeOwned,
        P: AsRef<Path>,
    {
        self.update_records(file_name, |mut records| {
            records.push(value);
            Ok(records)
        })
    }

    pub fn update_records<T, P, F>(
        &self,
        file_name: P,
        func: F,
    ) -> Result<TimedRecords<T>, FileError>
    where
        T: DeserializeOwned + Serialize,
        P: AsRef<Path>,
        F: FnOnce(TimedRecords<T>) -> UpdateResult<TimedRecords<T>>,
    {
        let path = self.data_file_for_write(&file_name)?;
        let mut records = load_records_or_default(&path)?;

        records = match func(records) {
            Ok(mut value) => {
                value.sort_by_timestamp();
                save_json(&value, &path)?;
                value
            }
            Err(value) => value,
        };

        Ok(records)
    }

    pub async fn update_records_async<T, P, F>(
        &self,
        file_name: P,
        func: F,
    ) -> Result<TimedRecords<T>, FileError>
    where
        T: DeserializeOwned + Serialize,
        P: AsRef<Path>,
        F: AsyncFnOnce(TimedRecords<T>) -> UpdateResult<TimedRecords<T>>,
    {
        let path = self.data_file_for_write(&file_name)?;
        let mut records = load_records_or_default(&path)?;

        records = match func(records).await {
            Ok(mut value) => {
                value.sort_by_timestamp();
                save_json(&value, &path)?;
                value
            }
            Err(value) => value,
        };

        Ok(records)
    }
}

fn load_records_or_default<T, P>(path: P) -> Result<TimedRecords<T>, FileError>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let records = if exists_and_non_empty(path) {
        load_json(path)?
    } else {
        Default::default()
    };

    Ok(records)
}
