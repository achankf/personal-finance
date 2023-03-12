use std::{fs, path::PathBuf, str::FromStr};

use sqlx::{
    migrate::MigrateError,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions, SqliteConnection,
};

use crate::{Db, SqlResult};

impl Db {
    fn path() -> PathBuf {
        let dir = home::home_dir()
            .expect("cannot find home directory")
            .join(".pit");
        fs::create_dir_all(dir.clone()).expect("cannot create directory for storing data");
        dir.join("data.sqlite3")
    }

    pub async fn new() -> SqlResult<Self> {
        let path = Self::path();
        let path = path.to_str().expect("to get the string for the path");
        let conn = SqliteConnectOptions::from_str(path)?
            .journal_mode(SqliteJournalMode::Wal)
            .foreign_keys(true)
            .optimize_on_close(true, None)
            .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Full)
            .create_if_missing(true)
            .connect()
            .await?;

        let mut db = Self(conn);
        db.migrate_to_latest().await?;

        Ok(db)
    }

    pub async fn reset() -> Result<(), Box<dyn std::error::Error>> {
        fs::remove_file(Self::path())?;
        Self::new().await?;
        Ok(())
    }

    pub async fn migrate_to_latest(&mut self) -> Result<(), MigrateError> {
        sqlx::migrate!("../migrations").run(&mut self.0).await
    }
}

impl std::ops::Deref for Db {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Db {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
