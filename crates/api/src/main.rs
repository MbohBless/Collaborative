mod errors;
mod middleware;
mod router;
mod state;

use axum::{routing::get, Router};
use state::AppState;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let config = config_lib::Config::from_env()?;
    let db_pool = db::create_pool(&config.database_url).await?;
    
    // Create auth service and app state
    let auth_service = core::AuthService::new(config.jwt_secret.clone(), config.jwt_expiration_hours);
    let app_state = AppState::new(db_pool, auth_service);
    let app=Router::new()
        .route("/", get(|| async { "Collaborative Platform API" }))
        .route("/health", get(health_check))
        .nest("/api", router::create_router(app_state))
        .layer(TraceLayer::new_for_http());
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}



// use tokio::net::TcpListener;
// use tower_http::trace::TraceLayer;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // Initialize tracing
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env()
//                 .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();

//     let config = config_lib::Config::from_env()?;
//     db::create_pool(&config.database_url).await?;

//     // Build router
//     let app = Router::new()
//         .route("/", get(|| async { "Collaborative Platform API" }))
//         .route("/health", get(health_check))
//         .layer(TraceLayer::new_for_http());

//     // Start server
//     let addr = format!("{}:{}", config.host, config.port);
//     let listener = TcpListener::bind(&addr).await?;
    
//     tracing::info!("Server listening on {}", addr);
    
//     axum::serve(listener, app).await?;

//     Ok(())
// }

// async fn health_check() -> &'static str {
//     "OK"
// }