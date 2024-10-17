mod account;
mod entity;
mod health;
mod middlewares;
mod project;
mod swagger;
mod user;
mod utils;
use crate::database::{self, PostgreDatabase};
use crate::external::External;
use crate::scheduler::Scheduler;
use health::health_checker_handler;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;
use crate::{AppState, Config};

use axum::{routing::get, Router};
use dotenv::dotenv;
use std::error::Error;
use std::sync::Arc;

pub async fn make_app() -> Result<Router, Box<dyn Error>> {
    if dotenv().is_err() {
        println!("Starting server without .env file.");
    }
    // Configure the tracing subscriber with a custom filter
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug"))
        .add_directive("selectors=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap())
        .add_directive("html5ever=off".parse().unwrap())
        .add_directive("hyper_util=off".parse().unwrap());

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::CLOSE)
            .with_filter(filter))
        .init();
    let config = Config::init();
    // configure_logger(&config.log_level);
    info!("Connecting to PostgreSQL...");
    let sqlx_db_connection = database::connect_sqlx(&config.db_url).await;
    info!("Connected to PostgreSQL!");

    //let cors = HeaderValue::from_str(&config.cors_url)?;
    // TODO: Consider readding CORS here
    //let cors = CorsLayer::new()
    //    .allow_origin(cors)
    //    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    //    .allow_credentials(true)
    //    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let db = PostgreDatabase::new(sqlx_db_connection);
    let ext = External::new();
    let scheduler = Scheduler::new(db.clone(), ext.clone());
    let state = Arc::new(AppState { db, ext, config });
    let ret = Router::new()
        .route("/api", get(health_checker_handler))
        .route("/api/health", get(health_checker_handler))
        .nest("/api/user", user::user_routes(state.clone()))
        .nest("/api/entity", entity::entity_routes(state.clone()))
        .nest("/api/account", account::account_routes(state.clone()))
        .nest("/api/project", project::project_routes(state.clone()))
        .nest("/api/utils", utils::utils_routes(state.clone()))
        .merge(swagger::build_documentation())
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    //.layer(cors);

    tokio::spawn(async move {
        scheduler.spawn_tasks().await;
    });
    Ok(ret)
}
