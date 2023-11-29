use dotenv::dotenv;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL_TRANSACTION").expect("DATABASE_URL_TRANSACTION must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
        .batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id SERIAL PRIMARY KEY,
                user_id INTEGER REFERENCES users(id) NOT NULL,
                amount DECIMAL NOT NULL
            );

            INSERT INTO transactions (user_id, amount) VALUES
                (1, 100.0),
                (2, -50.0);
        "#,
        )
        .await?;

    println!("Transaction Microservice is running.");

    Ok(())
}
