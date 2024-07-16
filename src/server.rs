use std::{
    env,
    io::{Read, Write},
};

use axum::{
    extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use colored::Colorize;
use tokio::net::TcpListener;

use crate::client::ClientFactory;

pub async fn create_server(client_factory: ClientFactory) -> anyhow::Result<(TcpListener, Router)> {
    let port: String = env::var("SERVER_PORT").unwrap_or("3000".to_string());
    let binding = format!("0.0.0.0:{}", port);
    println!("listeng at: {}", binding.green());

    // build our application with a single route
    let app = Router::new()
        .nest("/", make_api())
        .with_state(client_factory);

    let listener = tokio::net::TcpListener::bind(binding).await.unwrap();

    Ok((listener, app))
}

#[derive(Serialize, Deserialize)]
struct Question {
    pure: String,
}

#[derive(Serialize)]
struct ClientResponse {
    pure: String,
}

impl ClientResponse {
    pub fn new(pure: String) -> Self {
        Self { pure }
    }
}

fn make_api() -> Router<ClientFactory> {
    Router::new().route("/test", post(test))
}

// which calls one of these handlers
async fn test(
    extract::State(state): extract::State<ClientFactory>,
    extract::Json(question): Json<Question>,
) -> Response {
    let Ok(mut stream) = state.create_stream().await else {
        return StatusCode::FAILED_DEPENDENCY.into_response();
    };

    let _ = stream.write(question.pure.as_bytes());
    let mut res = String::new();
    let _ = stream.read_to_string(&mut res);

    Json(ClientResponse::new(res)).into_response()
}
