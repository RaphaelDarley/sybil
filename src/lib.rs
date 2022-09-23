pub mod db_utils;
pub mod http_handler;
pub mod ws_handler;

use db_utils::{make_or_load_ds_and_sess, DSConn};
use std::{convert::Infallible, sync::Arc};
use surrealdb::Datastore;
use tokio::sync::Mutex;
use warp::Filter;

pub struct InnerState {
    pub dsconn: DSConn,
}

// impl InnerState {
//     pub fn new_default() -> Self {
//         Self {
//             datastore: make_or_load_ds_and_sess("memory").await.unwrap(),
//         }
//     }
// }

pub type State = Arc<InnerState>;

pub fn add_state(state: State) -> impl Filter<Extract = (State,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}
