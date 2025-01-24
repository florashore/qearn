use server::{telemetry, Configuration, Db};
use tokio::net::TcpListener;
use server::service::rpc_service::fetch_tick_info;
use server::db::queries::create_tick_info;

#[tokio::main]
async fn main() {
    // Loads the .env file located in the environment's current directory or its parents in sequence.
    // .env used only for development, so we discard error in all other cases.
    dotenvy::dotenv().ok();

    // Tries to load tracing config from environment (RUST_LOG) or uses "debug".
    telemetry::setup_tracing();

    // Parse configuration from the environment.
    // This will exit with a help message if something is wrong.
    tracing::debug!("Initializing configuration");
    let cfg = Configuration::new();

    // Initialize db pool.
    tracing::debug!("Initializing db pool");
    let db = Db::new(&cfg.db_dsn, cfg.db_pool_max_size)
        .await
        .expect("Failed to initialize db");

    tracing::debug!("Running migrations");
    db.migrate().await.expect("Failed to run migrations");

     // Fetch tick info from the RPC endpoint
     match fetch_tick_info().await {
        Ok(tick_info) => {
            println!("Tick info: {:?}", tick_info);
            create_tick_info(&db.pool, tick_info.tick, tick_info.duration, tick_info.epoch, tick_info.initial_tick).await;
        }
        Err(err) => {
            tracing::error!("Failed to fetch tick info: {}", err);
            return;
        }
    }

    // Spin up our server.
    tracing::info!("Starting server on {}", cfg.listen_address);
    let listener = TcpListener::bind(&cfg.listen_address)
        .await
        .expect("Failed to bind address");
    let router = server::router(cfg, db);
    axum::serve(listener, router)
        .await
        .expect("Failed to start server")
}
