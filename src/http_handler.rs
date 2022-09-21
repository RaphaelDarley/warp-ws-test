use crate::ws_handler;
use warp::{Rejection, Reply};

pub async fn ws_handler(ws: warp::ws::Ws) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(|socket| ws_handler::ws_connection(socket)))
}
