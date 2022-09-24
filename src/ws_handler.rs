use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use warp::ws::WebSocket;
use warp::ws::{self, Message};

use crate::db_utils::{add_visit_record, db_visits_as_json, print_db_visits};
use crate::State;

pub async fn ws_connection(ws: WebSocket, state: State) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    while let Some(msg_result) = ws_rx.next().await {
        match msg_result {
            Ok(msg) => {
                // println!("{:?}", msg);
                // add_visit_record(&state.dsconn).await.unwrap();
                // print_db_visits(&state.dsconn).await.unwrap();
                // let msg = db_visits_as_json(&state.dsconn).await.unwrap();

                // match ws_tx.send(ws::Message::text(msg)).await {
                //     Ok(_) => {}
                //     Err(e) => println!("Websocket ERROR: {:?}", e),
                // }
                match msg.is_text() {
                    true => {
                        if let Some(reply) = handle_text_msg(msg, state.clone()).await {
                            match ws_tx.send(reply).await {
                                Ok(_) => {}
                                Err(e) => println!("Websocket ERROR: {:?}", e),
                            }
                        };
                    }
                    false => {
                        println!("Websocket ERROR: Binary message not supported");
                    }
                }
            }
            Err(e) => {
                println!("Websocket ERROR: {:?}", e);
                break;
            }
        }
    }
}

async fn handle_text_msg(message: Message, state: State) -> Option<Message> {
    let msg = message.to_str().unwrap();

    let req: WSReq = match serde_json::from_str(msg) {
        Ok(r) => r,
        Err(e) => {
            println!("JSON parsing ERROR: {:?}", e);
            return None;
        }
    };

    println!("{:?}", req);

    match req.cmd {
        Cmd::Create => handle_create(req, state).await,
        Cmd::Read => {
            return Some(Message::text(
                db_visits_as_json(&state.dsconn).await.unwrap(),
            ))
        }
        _ => {}
    }

    // Some(Message::text("Test reply"))
    None
}

async fn handle_create(req: WSReq, state: State) {
    state
        .dsconn
        .execute("CREATE visit SET time = time::now()", None, false)
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct WSReq {
    cmd: Cmd,
    item_id: Option<String>,
    item_text: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Cmd {
    Create,
    Read,
    Update,
    Delete,
}
