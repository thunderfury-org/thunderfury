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

fn setup_sqlite_url(config_dir: &str) -> String {
    let db_dir = format!("{config_dir}/db");
    utils::fs::create_dir_all(&db_dir).unwrap();
    format!("sqlite:{db_dir}/thunderfury.db?mode=rwc")
}

async fn init_db(manager: &Manager) -> io::Result<DatabaseConnection> {
    let db_url = manager
        .get_server_config()
        .db_url
        .as_deref()
        .map_or_else(|| setup_sqlite_url(manager.get_config_dir()), |u| u.to_string());

    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false).max_connections(10);

    Ok(Database::connect(opt).await.expect("database connection failed"))
}

#[actix_web::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Server(args) => {
            run_server(Manager::try_from(args.config_dir.trim()).unwrap()).await;
        }
        Commands::Migrate(args) => {
            let manager = Manager::try_from(args.config_dir.trim()).unwrap();
            let db = init_db(&manager).await.unwrap();
            migration::fresh(&db).await.unwrap();
        }
    }
}

async fn run_server(manager: Manager) {
    let file_appender =
        tracing_appender::rolling::daily(format!("{}/log", manager.get_config_dir()), "thunderfury.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    logger::init(non_blocking);

    let db = init_db(&manager).await.unwrap();
    migration::up(&db).await.unwrap();

    let state = AppState {
        db,
        http_client: reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("failed to create http client"),
        config: manager,
    };

    // start background job
    job::start_job(&state);

    // start http server
    run_http_server(&state).await.unwrap();
}

async fn run_http_server(state: &AppState) -> io::Result<()> {
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
