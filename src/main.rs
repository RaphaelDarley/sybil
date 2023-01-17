use std::{net::SocketAddr, sync::Arc};

use crate::http_handler;
use axum::{routing::get, Router};
use sybil::{db_utils::make_or_load_ds_and_sess, *};
use tokio::sync::Mutex;
// use warp::Filter;

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(InnerState {
        // dsconn: make_or_load_ds_and_sess("file://database").await.unwrap(),
        dsconn: make_or_load_ds_and_sess("memory").await.unwrap(),
    });

    // let ws_route = warp::path("ws")
    //     .and(warp::ws())
    //     .and(add_state(state))
    //     .and_then(http_handler::ws_handler);

    // let static_route = warp::path("site").and(warp::fs::dir("website/public"));

    // let routes = ws_route
    //     .or(static_route)
    //     .with(warp::cors().allow_any_origin());

    // warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;

    let static_app = Router::new().route("/", get(http_handler::static_content));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(static_app.into_make_service())
        .await
        .unwrap();

    let ws_app = Router::new().route(
        "/*",
        get(|ws| async move { http_handler::ws_handler(ws, state.clone()).await }),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    axum::Server::bind(&addr)
        .serve(ws_app.into_make_service())
        .await
        .unwrap();
}
