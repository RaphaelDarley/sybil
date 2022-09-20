use futures_util::{FutureExt, StreamExt};
use std::sync::{Arc, Mutex};
use warp;
use warp::Filter;

#[tokio::main]
async fn main() {
    let state: handler::State = Arc::new(Mutex::new(0));

    let routes = warp::path("test")
        .and(warp::ws())
        .and(warp::any().map(move || state.clone()))
        .and_then(handler::test_handler);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

// let state = state.clone();
// || async move { handler::test_handler(state, websocket).await }

mod handler {
    use std::sync::{Arc, Mutex};

    use futures_util::{FutureExt, StreamExt};
    use warp::{ws::WebSocket, Rejection, Reply};

    pub type State = Arc<Mutex<i64>>;
    pub type Result<T> = std::result::Result<T, Rejection>;

    pub async fn test_handler(ws: warp::ws::Ws, state: State) -> Result<impl Reply> {
        Ok(ws.on_upgrade(move |socket| ws_handler(socket, state)))
    }

    async fn ws_handler(ws: WebSocket, state: State) {
        let (tx, rx) = ws.split();
    }
}
