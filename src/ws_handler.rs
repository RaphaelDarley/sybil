use futures_util::{SinkExt, StreamExt};
use warp::ws;
use warp::ws::WebSocket;

use crate::db_utils::{add_visit_record, print_db_entries};
use crate::State;

pub async fn ws_connection(ws: WebSocket, state: State) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    while let Some(msg_result) = ws_rx.next().await {
        match msg_result {
            Ok(msg) => {
                println!("{:?}", msg);
                // *state.request_count.write().await += 1;
                add_visit_record(&state.dsconn).await.unwrap();
                print_db_entries(&state.dsconn).await.unwrap();

                match ws_tx
                    .send(ws::Message::text(format!(
                        "you are visitor",
                        // state.request_count.read().await
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
