use tokio::net::TcpListener;
use tokio_cron_scheduler::{JobScheduler, Job};
use std::sync::Arc;

use api_server::{telemetry, Configuration};
use api_server::cronjob::cronjob;
use api_server::db::prisma::PrismaClient;

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
    let cfg: Arc<Configuration> = Configuration::new();

    let prisma_client: Arc<PrismaClient> = Arc::new(PrismaClient::_builder().build().await.unwrap());
    // Create the job scheduler
    let scheduler: JobScheduler = JobScheduler::new()
        .await
        .expect("Failed to create job scheduler");

    // Use `Job::new_async` for async closures
    let job = Job::new_async("1/10 * * * * *", move |_uuid: uuid::Uuid, _l: JobScheduler| {
        let value = prisma_client.clone();
        Box::pin(async move {
            let client = value.clone();
            if let Err(e) = cronjob(&client).await {
                eprintln!("Failed to run cronjob: {:?}", e);
            }
        })
    }).expect("Failed to create async job");

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
    let router = api_server::router(cfg);
    axum::serve(listener, router)
        .await
        .expect("Failed to start server");
}
