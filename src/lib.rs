pub mod http_handler;
pub mod ws_handler;

use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::Filter;

pub struct InnerState {
    pub request_count: usize,
}

impl InnerState {
    pub fn new_default() -> Self {
        Self { request_count: 0 }
    }
}

pub type State = Arc<Mutex<InnerState>>;

pub fn add_state(state: State) -> impl Filter<Extract = (State,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}
