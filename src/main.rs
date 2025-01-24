use server::{telemetry, Configuration};
use tokio::net::TcpListener;
use server::cronjob::cronjob;

use tokio_cron_scheduler::{JobScheduler, Job};

use server::db::prisma::PrismaClient;
use std::sync::Arc;

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

    // // Initialize db pool.
    // tracing::debug!("Initializing db pool");
    // let db = Db::new(&cfg.db_dsn, cfg.db_pool_max_size)
    //     .await
    //     .expect("Failed to initialize db");

    // tracing::debug!("Running migrations");
    // db.migrate().await.expect("Failed to run migrations");

    let prisma_client = Arc::new(PrismaClient::_builder().build().await.unwrap());

    #[cfg(debug_assertions)]
    prisma_client._db_push().await.unwrap();

    // Create the job scheduler
    let scheduler = JobScheduler::new()
        .await
        .expect("Failed to create job scheduler");

    // Use `Job::new_async` for async closures
    let job = Job::new_async("1/10 * * * * *", move |_uuid, _l| {
        Box::pin(async move {
            cronjob(&prisma_client)
                .await
                .expect("Failed to run cronjob");
        })
    })
    .expect("Failed to create async job");

    // Add the job to the scheduler
    scheduler
        .add(job)
        .await
        .expect("Failed to add job to the scheduler");

    // Start the scheduler
    scheduler.start().await.expect("Failed to start scheduler");

    // Axum server
    tracing::info!("Starting server on {}", cfg.listen_address);
    let listener = TcpListener::bind(&cfg.listen_address)
        .await
        .expect("Failed to bind address");
    let router = server::router(cfg);
    axum::serve(listener, router)
        .await
        .expect("Failed to start server");
}
