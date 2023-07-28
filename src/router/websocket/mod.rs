mod online_websocket;

use crate::router::websocket::online_websocket::OnLineWebSocket;
use crate::server;
use actix::Addr;
use actix_http::ws::Codec;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix_web_actors::ws::{HandshakeError, WebsocketContext};
use std::collections::HashMap;
use std::time::Instant;

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(
    r: HttpRequest,
    info: web::Query<HashMap<String, String>>,
    stream: web::Payload,
    srv: web::Data<Addr<server::OnLineServer>>,
) -> Result<HttpResponse, HandshakeError> {
    log::info!("新连接·····");
    let room_id = &info.0["room"];
    let mut res = ws::handshake(&r)?;
    Ok(res.streaming(WebsocketContext::with_codec(
        OnLineWebSocket {
            id: 0,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
            room_id: room_id.parse::<u16>().unwrap(),
        },
        stream,
        Codec::new().max_size(1024 * 1024 * 6),
    )))
    // let res = ws::start(OnLineWebSocket {
    //     id: 0,
    //     hb: Instant::now(),
    //     addr: srv.get_ref().clone(),
    // }, &r, stream);
    // Ok(res.expect(""))
}
