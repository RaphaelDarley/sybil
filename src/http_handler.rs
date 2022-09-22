use crate::{ws_handler, State};
use warp::{Rejection, Reply};

pub async fn ws_handler(ws: warp::ws::Ws, state: State) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(|socket| ws_handler::ws_connection(socket, state)))
}
