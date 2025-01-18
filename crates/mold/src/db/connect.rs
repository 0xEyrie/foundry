use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, Pool, Postgres,
};
use std::sync::Arc;

const DB_NAME: &str = "indexer";
const PORT: u16 = 5432;

pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn connect(
        endpoint: &String,
        username: &String,
        pw: &String,
    ) -> Result<Database, String> {
        let connect_options = PgConnectOptions::new()
            .username(username)
            .password(pw)
            .port(PORT)
            .host(endpoint)
            .database(DB_NAME)
            .ssl_mode(PgSslMode::Prefer);
        let url = connect_options.to_url_lossy();

        println!("Connecting Postgres, url: {}", url);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(connect_options)
            .await
            .expect(&format!("url: {}", url));

        let d = Database { pool };

        return Ok(d);
    }
}

pub async fn connect_db() -> Arc<Database> {
    let db = {
        let pg_endpoint = String::from("localhost");
        let pg_username = String::from("postgres");
        let pg_password = String::from("postgres");

        let db = Database::connect(&pg_endpoint, &pg_username, &pg_password).await.unwrap();
        Arc::new(db)
    };

    db
}
