use std::collections::BTreeMap;

use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde::Deserialize;
use surrealdb::sql::{Strand, Thing, Value};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::Message;
use warp::ws::WebSocket;

use crate::db_utils::{add_visit_record, db_visits_as_json, print_db_visits};
use crate::item::Item;
use crate::State;

pub async fn ws_connection(ws: WebSocket, state: State) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::spawn(async move {
        while let Some(message) = rx.next().await {
            ws_tx
                .send(message)
                .unwrap_or_else(|e| println!("Websocket error: {}", e))
                .await;
        }
    });

    while let Some(msg_result) = ws_rx.next().await {
        match msg_result {
            Ok(msg) => {
                if msg.is_text() {
                    // if let Some(reply) = handle_text_msg(msg, state.clone()).await {
                    //     match ws_tx.send(reply).await {
                    //         Ok(_) => {}
                    //         Err(e) => println!("Websocket ERROR: {:?}", e),
                    //     }
                    // };
                    tokio::spawn(handle_text_msg(msg, state.clone(), tx.clone()));
                } else if msg.is_binary() {
                    println!("Websocket ERROR: Binary message not supported");
                    println!("{:?}", msg);
                } else if msg.is_close() {
                    return;
                }
            }
            Err(e) => {
                println!("Websocket ERROR: {:?}", e);
                break;
            }
        }
    }
}

async fn handle_text_msg(
    message: Message,
    state: State,
    tx: UnboundedSender<Message>,
) -> Option<Message> {
    let msg = message.to_str().unwrap();

    let req: Req = match serde_json::from_str(msg) {
        Ok(r) => r,
        Err(e) => {
            println!("JSON parsing ERROR: {:?}", e);
            return None;
        }
    };

    println!("{:?}", req);

    match req {
        Req::Create(c) => handle_create(c, state, tx).await,
        Req::Read(r) => handle_read(r, state, tx).await,
        Req::Update(u) => handle_update(u, state, tx).await,
        _ => None,
    }
}

async fn handle_create(
    req: CreateReq,
    state: State,
    tx: UnboundedSender<Message>,
) -> Option<Message> {
    let mut vars = BTreeMap::new();
    // vars.insert(
    //     "text".to_string(),
    //     surrealdb::sql::Value::Strand(Strand::from(req.text)),
    // );

    req.item.add_to_vars(&mut vars);

    let set_query = vars
        .iter()
        .map(|(k, _)| format!("{} = ${}", k, k))
        .collect::<Vec<String>>()
        .join(", ");

    state
        .dsconn
        .execute(
            "CREATE item SET time_created = time::now(), text = $text",
            Some(vars),
            false,
        )
        .await
        .unwrap();

    //from read handler sends back rows with changes
    let select_response = state
        .dsconn
        .execute("SELECT * FROM item", None, false)
        .await
        .unwrap();

    let select_result = select_response[0].output().unwrap();

    if let Value::Array(rows) = select_result {
        let reply = Message::text(serde_json::to_string(rows).unwrap());
        println!("{:?}", reply);
        tx.send(reply).unwrap();
        None
    } else {
        panic!("DB vists hasn't returned array of rows")
    }
}

async fn handle_read(req: ReadReq, state: State, tx: UnboundedSender<Message>) -> Option<Message> {
    let mut vars = BTreeMap::new();
    let mut where_stmt = vec![];

    if let Some(text_search) = req.text_search {
        vars.insert(
            "text_search".to_string(),
            surrealdb::sql::Value::Strand(Strand::from(text_search)),
        );
        where_stmt.push("text ~ $text_search");
    }

    let where_stmt = where_stmt.join(" && ");

    let mut query = "SELECT * FROM item".to_string();

    if where_stmt != "" {
        query += " WHERE ";
        query += &where_stmt;
    }

    query += ";";

    println!("{}", query);
    println!("{:?}", vars);

    let select_response = state
        .dsconn
        .execute(&query, Some(vars), false)
        .await
        .unwrap();

    let select_result = select_response[0].output().unwrap();

    if let Value::Array(rows) = select_result {
        let reply = Message::text(serde_json::to_string(rows).unwrap());
        println!("{:?}", reply);
        tx.send(reply).unwrap();
        None
    } else {
        panic!("DB vists hasn't returned array of rows")
    }
}

async fn handle_update(
    req: UpdateReq,
    state: State,
    tx: UnboundedSender<Message>,
) -> Option<Message> {
    let mut vars = BTreeMap::new();
    vars.insert(
        "text".to_string(),
        surrealdb::sql::Value::Strand(Strand::from(req.text)),
    );
    let record = req.id.split_once(":").unwrap();
    vars.insert(
        "record".to_string(),
        surrealdb::sql::Value::Thing(Thing::from(record)),
    );
    state
        .dsconn
        .execute("UPDATE $record SET text = $text", Some(vars), false)
        .await
        .unwrap();

    //from read handler sends back rows with changes
    let select_response = state
        .dsconn
        .execute("SELECT * FROM item", None, false)
        .await
        .unwrap();

    let select_result = select_response[0].output().unwrap();

    if let Value::Array(rows) = select_result {
        let reply = Message::text(serde_json::to_string(rows).unwrap());
        println!("{:?}", reply);
        tx.send(reply).unwrap();
        None
    } else {
        panic!("DB vists hasn't returned array of rows")
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Req {
    Create(CreateReq),
    Read(ReadReq),
    Update(UpdateReq),
    Delete,
}

#[derive(Deserialize, Debug)]
struct CreateReq {
    item: Item,
}

#[derive(Deserialize, Debug)]
struct ReadReq {
    text_search: Option<String>,
}

#[derive(Deserialize, Debug)]
struct UpdateReq {
    id: String,
    text: String,
}
