use std::io;

use actix_web::{web, App, HttpServer};
use clap::Parser;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

use cli::{Cli, Commands};
use common::{config::Manager, state::AppState};

#[cfg(test)]
mod test;

mod api;
mod cli;
mod common;
mod entity;
mod job;
mod library;
mod logger;
mod migration;
mod subscription;
mod task;
mod utils;

async fn init_db(config_dir: &str) -> io::Result<DatabaseConnection> {
    if config_dir.is_empty() {
        panic!("config dir is empty");
    }

    let db_dir = format!("{}/db", config_dir);
    utils::fs::create_dir_all(&db_dir)?;

    let url = format!("sqlite:{}/thunderfury.db?mode=rwc", db_dir);

    let mut opt = ConnectOptions::new(url);
    opt.sqlx_logging(false).max_connections(10);

    Ok(Database::connect(opt).await.expect("database connection failed"))
}

#[actix_web::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Server(args) => {
            run_server(args.config_dir.trim()).await;
        }
        Commands::Migrate(args) => {
            let db = init_db(args.config_dir.trim()).await.unwrap();
            migration::fresh(&db).await.unwrap();
        }
    }
}

async fn run_server(config_dir: &str) {
    let file_appender = tracing_appender::rolling::daily(format!("{}/log", config_dir), "thunderfury.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    logger::init(non_blocking);

    let db = init_db(config_dir).await.unwrap();
    migration::up(&db).await.unwrap();

    let state = AppState {
        db,
        http_client: reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("failed to create http client"),
        config: Manager::try_from(format!("{}/config.yaml", config_dir).as_str()).unwrap(),
    };

    // start background job
    job::start_job(&state);

    // start http server
    run_http_server(&state).await.unwrap();
}

async fn run_http_server(state: &AppState) -> std::io::Result<()> {
    let addr = state.config.get_server_address();
    info!("server starting on {}", addr);

    let state = web::Data::new(state.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(logger::custom_http_logger())
            .service(web::resource("/health").to(|| async { "I am working!" }))
            .configure(api::api)
    })
    .bind(addr)?
    .run()
    .await
}
