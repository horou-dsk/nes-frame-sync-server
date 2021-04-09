mod websocket;

use actix_web::web;
use serde::{Deserialize, Serialize};
use crate::router::websocket::ws_index;

#[derive(Serialize, Deserialize)]
struct ResultOk<T> {
    code: u16,
    data: T,
}

impl<T> ResultOk<T> {
    fn new(data: T) -> Self {
        ResultOk { code: 200, data }
    }
}

#[derive(Serialize, Deserialize)]
struct ResultErr {
    code: u16,
    err_msg: String,
}

impl ResultErr {
    fn new(code: u16, err_msg: String) -> Self {
        ResultErr { code, err_msg }
    }
}


pub struct ResultJson;

impl ResultJson {
    fn ok<T>(data: T) -> ResultOk<T> {
        ResultOk::new(data)
    }

    fn err<S: Into<String>>(code: u16, err_msg: S) -> ResultErr {
        ResultErr::new(code, err_msg.into())
    }
}

pub fn router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws").route(web::get().to(ws_index)));
}