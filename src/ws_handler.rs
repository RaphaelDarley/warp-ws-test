use futures::StreamExt;
use warp::ws::WebSocket;

pub async fn ws_connection(ws: WebSocket) {
    let (tx, rx) = ws.split();

    while let Some(msg) = rx.next() {}
}
