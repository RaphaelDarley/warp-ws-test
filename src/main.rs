use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;
use warp_ws_test::http_handler;
use warp_ws_test::*;

#[tokio::main]
async fn main() {
    let state: State = Arc::new(Mutex::new(InnerState::new_default()));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(add_state(state))
        .and_then(http_handler::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}
