use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    path::PathBuf,
};

use common::{deserialize_into_map, Id};
use owo_colors::OwoColorize;
use serde::de;
use sqlx::Sqlite;

use db::{Query, SqlQuery, SqlResult};

use crate::Transaction;

impl Transaction<'_> {
    pub async fn commit(self) -> SqlResult<()> {
        self.0.commit().await
    }

    pub async fn rollback(self) -> SqlResult<()> {
        self.0.rollback().await
    }

    pub async fn execute(
        &mut self,
        query: SqlQuery<'_>,
    ) -> SqlResult<<Sqlite as sqlx::Database>::QueryResult> {
        Ok(query.execute(&mut *self.0).await?)
    }

    pub async fn upsert_all<T>(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<BTreeMap<T::IdType, T>, Box<dyn std::error::Error>>
    where
        T: Id + de::DeserializeOwned + Query + std::fmt::Debug,
    {
        print!(
            "{}: updating data with {}... ",
            "Info".bold().yellow(),
            csv_path.to_string_lossy()
        );

        let parsed_records = deserialize_into_map::<T>(csv_path)?;

        let mut num_rows = 0;
        let mut num_rows_updated = 0;
        for (_, record) in &parsed_records {
            match self.execute(record.query()).await {
                Ok(result) => {
                    num_rows += 1;
                    if result.rows_affected() == 1 {
                        num_rows_updated += 1;
                    }
                }
                Err(err) => return Err(format!("{}, record: {:?}", err.to_string(), record).into()),
            }
            if let Err(err) = self.execute(record.query()).await {
                return Err(format!("{}, record: {:?}", err.to_string(), record).into());
            }
        }
        println!(
            "number of rows updated/total: {}/{}",
            num_rows_updated, num_rows
        );

        Ok(parsed_records)
    }

    pub async fn upsert_all_in_order<T>(
        &mut self,
        csv_path: &PathBuf,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: Id + de::DeserializeOwned + Query + std::fmt::Debug,
    {
        print!("updating data with {}", csv_path.to_string_lossy());

        let mut parsed_records = Vec::new();

        let file = File::open(csv_path)?;
        let mut set = BTreeSet::new();

        for result in csv::Reader::from_reader(file).deserialize::<T>() {
            let record = result?;
            let id = record.id();

            if !set.insert(id.clone()) {
                println!();
                return Err(format!("deserialization failed due to duplicated id: {id:#?}").into());
            }

            parsed_records.push(record);
        }

        let mut num_rows = 0;
        let mut num_rows_updated = 0;
        for record in &parsed_records {
            match self.execute(record.query()).await {
                Ok(result) => {
                    num_rows += 1;
                    if result.rows_affected() == 1 {
                        num_rows_updated += 1;
                    }
                }
                Err(err) => {
                    return Err(format!("{}, record: {:?}", err.to_string(), record).into());
                }
            }
        }
        println!(
            "number of rows updated/total: {}/{}",
            num_rows_updated, num_rows
        );

        Ok(parsed_records)
    }
}

impl<'c> std::ops::Deref for Transaction<'c> {
    type Target = sqlx::Transaction<'c, sqlx::Sqlite>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Transaction<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
