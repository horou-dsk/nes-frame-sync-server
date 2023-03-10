mod online_websocket;

use std::collections::HashMap;
use std::time::Instant;
use actix_web_actors::ws;
use actix_web_actors::ws::{WebsocketContext, HandshakeError};
use actix_web::{HttpRequest, web, HttpResponse};
use log::{info};
use actix::Addr;
use crate::server;
use crate::router::websocket::online_websocket::OnLineWebSocket;
use actix_http::ws::Codec;

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(r: HttpRequest, info: web::Query<HashMap<String, String>>, stream: web::Payload, srv: web::Data<Addr<server::OnLineServer>>) -> Result<HttpResponse, HandshakeError> {
    info!("新连接·····");
    let room_id = &info.0["room"];
    let mut res = ws::handshake(&r)?;
    Ok(res.streaming(WebsocketContext::with_codec(OnLineWebSocket {
        id: 0,
        hb: Instant::now(),
        addr: srv.get_ref().clone(),
        room_id: room_id.parse::<u16>().unwrap(),
    }, stream, Codec::new().max_size(1024 * 1024 * 6))))
    // let res = ws::start(OnLineWebSocket {
    //     id: 0,
    //     hb: Instant::now(),
    //     addr: srv.get_ref().clone(),
    // }, &r, stream);
    // Ok(res.expect(""))
}
