mod db;

use sqlx::SqliteConnection;

pub type SqlResult<T> = sqlx::Result<T>;

pub struct Db(SqliteConnection);

pub type SqlQuery<'q> = sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>>;

pub trait Query {
    fn query(&self) -> SqlQuery;
}
