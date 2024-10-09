mod config;
mod game;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use config::Config;
use game::{start_game, update_game};
use std::sync::Arc;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// TODO: Add unit that allows us to get a game from storage
pub struct AppState {
    config: Config,
}

#[tokio::main]
async fn main() {
    // Start the tracer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "laika_th=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Initialize our config from environment variables
    let config = Config::from_env();
    // Create our listener on configured address
    let listener = tokio::net::TcpListener::bind(&config.get_address())
        .await
        .unwrap();
    // Store shared data as state between routes
    let state = Arc::new(AppState { config });
    // Initialize the api routes
    let app = Router::new()
        .route("/", get(index_page))
        .route("/game", post(start_game))
        .route("/game/:game_id", get(update_game))
        .with_state(state)
        // Add a trace layer to trace response and request times
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Seconds),
                ),
        );
    // Start the server
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index_page() -> impl IntoResponse {
    "Tic tac toe coming soon!"
}
