use ethers::core::types::Address;
use hex;
use log::info;
use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};

pub struct DbManager {
    db: SqlitePool,
}

const DB_URL: &str = "sqlite://.aeloc.db";

impl DbManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            info!("Creating database {}", DB_URL);
            Sqlite::create_database(DB_URL).await?;
        }

        let db = SqlitePool::connect(DB_URL).await?;
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
