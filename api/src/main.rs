mod database;
mod game_data_api;
mod game_data_service;
mod utils;
use chrono::DateTime;
use game_data_api::game_data_server::GameDataServer;
use game_data_service::GameDataService;
use sonyflake::Builder;
use sqlx::SqlitePool;
use std::env;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    let database_url =
        env::var("DATABASE_URL").expect("Environment variable 'DATABASE_URL' not found.");
    let db = SqlitePool::connect(&database_url)
        .await
        .expect("Could not load SQLite database.");

    let machine_id_func: &dyn Fn() -> Result<
        u16,
        Box<(dyn std::error::Error + Send + Sync + 'static)>,
    > = &|| {
        Ok(env::var("MACHINE_ID")
            .map_err(|_e| "Environment variable 'MACHINE_ID' not found.")?
            .parse::<u16>()
            .map_err(|_e| "Environment variable 'MACHINE_ID' is not a 16 bit integer.")?)
    };
    let sf = Builder::new()
        .start_time(DateTime::UNIX_EPOCH)
        .machine_id(machine_id_func)
        .finalize()
        .expect("Failed to initialize ID generator");

    let addr = "[::1]:50051".parse()?;
    println!(
        "☄️ Starting Project Comet Game Data API Service on: http://{}",
        addr
    );
    let service = GameDataService::new(db, sf);
    Server::builder()
        .add_service(GameDataServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
