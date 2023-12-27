use crate::config::CONFIG;
use crate::error::Error;
use crate::{DBCon, DBPool};
use r2d2_oracle::{r2d2, OracleConnectionManager};
use std::fs;

pub type Result<T> = std::result::Result<T, Error>;

const DB_POOL_MAX_OPEN: u32 = 32;
const CREATE_TABLES_SQL: &str = "./db-scripts/tables.sql";
const SEEDING_DATA_SQL: &str = "./db-scripts/seeding.sql";
const CREATE_PROCEDURES_SQL: &str = "./db-scripts/procedures.sql";
const CREATE_TRIGGERS_SQL: &str = "./db-scripts/triggers.sql";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let con = get_db_con(db_pool).await?;

    let init_file = fs::read_to_string(CREATE_TABLES_SQL)?;
    con.execute(init_file.as_str(), &[])
        .map_err(Error::DBInit)?;

    let init_file = fs::read_to_string(SEEDING_DATA_SQL)?;
    con.execute(init_file.as_str(), &[])
        .map_err(Error::DBInit)?;

    let init_file = fs::read_to_string(CREATE_PROCEDURES_SQL)?;
    con.execute(init_file.as_str(), &[])
        .map_err(Error::DBInit)?;

    let init_file = fs::read_to_string(CREATE_TRIGGERS_SQL)?;
    con.execute(init_file.as_str(), &[])
        .map_err(Error::DBInit)?;

    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().map_err(Error::DBPool)
}

pub fn create_pool() -> std::result::Result<DBPool, oracle::Error> {
    let config = CONFIG.clone();
    let manager = OracleConnectionManager::new(
        config.db_user.as_str(),
        config.db_pass.as_str(),
        config.db_dsn.as_str(),
    );

    Ok(r2d2::Pool::builder()
        .max_size(DB_POOL_MAX_OPEN)
        .build(manager)
        .expect("database pool can be created"))
}
