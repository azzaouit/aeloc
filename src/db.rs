use dirs::config_dir;
use ethers::core::types::Address;
use hex;
use log::info;
use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};
use std::path::PathBuf;

const DB_NAME: &str = "aeloc.db";

pub struct DbManager {
    db: SqlitePool,
}

impl DbManager {
    pub fn db_dir() -> PathBuf {
        config_dir().unwrap().join("aeloc")
    }

    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_dir = Self::db_dir();
        let db_path = db_dir.join(DB_NAME).into_os_string().into_string().unwrap();

        if !Sqlite::database_exists(&db_path).await.unwrap_or(false) {
            let _ = std::fs::create_dir(db_dir);
            Sqlite::create_database(&db_path).await?;
        }

        let db = SqlitePool::connect(&db_path).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS authorized (id INTEGER PRIMARY KEY, address VARCHAR(64) UNIQUE);")
            .execute(&db)
            .await?;
        Ok(Self { db })
    }

    pub async fn authorize_contract(&self, a: Address) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO authorized (address) values ($1)")
            .bind(hex::encode(a))
            .execute(&self.db)
            .await?;

        info!("Address {} authorized", hex::encode(a));
        Ok(())
    }

    pub async fn remove_contract(&self, a: Address) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM authorized WHERE address=$1")
            .bind(hex::encode(a))
            .execute(&self.db)
            .await?;

        info!("Address {} removed", hex::encode(a));
        Ok(())
    }

    pub async fn authorized_contracts(&self) -> Result<Vec<Address>, sqlx::Error> {
        let auth = sqlx::query("SELECT address FROM authorized")
            .fetch_all(&self.db)
            .await?;
        let res: Vec<Address> = auth
            .iter()
            .map(|r| {
                let addr: &str = r.get(0);
                addr.parse::<Address>().unwrap()
            })
            .collect();
        info!("Fetched {} authorized contracts", res.len());
        Ok(res)
    }
}
