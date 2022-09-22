use futures_util::{SinkExt, StreamExt};
use warp::ws;
use warp::ws::WebSocket;

use crate::State;

pub async fn ws_connection(ws: WebSocket, state: State) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    while let Some(msg_result) = ws_rx.next().await {
        match msg_result {
            Ok(msg) => {
                println!("{:?}", msg);
                state.lock().await.request_count += 1;
                match ws_tx
                    .send(ws::Message::text(format!(
                        "you are visitor: {}",
                        state.lock().await.request_count
                    )))
                    .await
                {
                    Ok(_) => {}
                    Err(e) => println!("Websocket ERROR: {:?}", e),
                }
            }
            Err(e) => {
                println!("Websocket ERROR: {:?}", e);
                break;
            }
        }
    }
}
