use crate::{ws_handler, AppState};
// use warp::{Rejection, Reply};
use axum::{extract::ws::WebSocketUpgrade, response::IntoResponse};

pub async fn ws_handler(ws: WebSocketUpgrade, state: AppState) -> impl IntoResponse {
    ws.on_upgrade(|socket| ws_handler::ws_connection(socket, state))
}

pub async fn static_content() -> &'static str {
    "Hello World"
}
