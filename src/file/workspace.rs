use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use clap::Args;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::FileError;
use super::ops::{load_json, make_directory, save_html, save_json};
use super::url::url_to_file_name;
use crate::web::page::Page;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimedRecord<T> {
    pub timestamp: DateTime<Utc>,
    pub data: T,
}

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

#[derive(Clone, Debug, Args)]
pub struct Workspace {
    /// The root directory of the workspace
    #[arg(long = "workspace", short = 'w', default_value = ".rentmap")]
    pub root: PathBuf,
}

impl Workspace {
    pub fn data(&self) -> PathBuf {
        self.root.join("data")
    }

    pub fn html(&self) -> PathBuf {
        self.root.join("html")
    }

    pub fn init(&self) -> Result<(), FileError> {
        make_directory(&self.root)
    }

    pub fn save_page(&self, page: &Page) -> Result<(), FileError> {
        let file_name = url_to_file_name(&page.url_final);
        let mut path = self.html();
        make_directory(&path)?;
        path.push(&file_name);
        save_html(&page.html, path)
    }

    pub fn save_timed_records<T, P>(
        &self,
        data: &Vec<TimedRecord<T>>,
        file_name: P,
    ) -> Result<(), FileError>
    where
        T: Serialize,
        P: AsRef<Path>,
    {
        let mut path = self.data();
        make_directory(&path)?;
        path.push(file_name);
        save_json(data, &path)?;
        Ok(())
    }

    pub fn load_timed_records<T, P>(&self, file_name: P) -> Result<Vec<TimedRecord<T>>, FileError>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        let mut path = self.data();
        path.push(file_name);
        let mut records: Vec<TimedRecord<T>> = load_json(&path)?;
        records.sort_unstable_by_key(|r| r.timestamp);
        Ok(records)
    }

    pub fn add_timed_record<T, P>(
        &self,
        data: T,
        file_name: P,
    ) -> Result<Vec<TimedRecord<T>>, FileError>
    where
        T: Serialize + DeserializeOwned,
        P: AsRef<Path>,
    {
        let mut path = self.data();
        make_directory(&path)?;

        path.push(file_name);

        let record = TimedRecord::now(data);

        let records = if path.exists() {
            let mut records = load_json(&path)?;
            sort_and_insort_by_timestamp(&mut records, record);
            records
        } else {
            vec![record]
        };

        save_json(&records, &path)?;

        Ok(records)
    }
}

fn sort_and_insort_by_timestamp<T>(
    records: &mut Vec<TimedRecord<T>>,
    record: TimedRecord<T>,
) -> usize
where
    T: Serialize + DeserializeOwned,
{
    records.sort_unstable_by_key(|r| r.timestamp);

    let index = records
        .binary_search_by_key(&record.timestamp, |r| r.timestamp)
        .unwrap_or_else(|s| s);

    records.insert(index, record);

    index
}
