use futures::StreamExt;
use warp::ws::WebSocket;

pub async fn ws_connection(ws: WebSocket) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    while let Some(msg_result) = ws_rx.next().await {
        match msg_result {
            Ok(msg) => println!("{:?}", msg),
            Err(e) => {
                println!("Websocket ERROR: {:?}", e);
                break;
            }
        }
    }
}
