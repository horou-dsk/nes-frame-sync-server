mod params;

use crate::router::nes::params::GameName;
use crate::router::ResultJson;
use crate::server;
use crate::server::messages::{CreateRoom, GetRoomInfo};
use actix::Addr;
use actix_web::{get, web, Error, HttpResponse};
use log::info;
use std::collections::HashMap;
use std::option::Option::Some;

type HttpResultResp = Result<HttpResponse, Error>;

#[get("/create_room")]
pub async fn create_room(
    info: web::Query<GameName>,
    srv: web::Data<Addr<server::OnLineServer>>,
) -> HttpResultResp {
    let game = info.0.game;
    let resp: u16 = srv
        .get_ref()
        .send(CreateRoom(game))
        .await
        .expect("create room error!");
    Ok(HttpResponse::Ok().json(ResultJson::ok(resp)))
}

#[get("/join_room")]
pub async fn join_room(
    info: web::Query<HashMap<String, String>>,
    srv: web::Data<Addr<server::OnLineServer>>,
) -> HttpResultResp {
    let room = info.0["room"].clone();
    let room = room.parse::<u16>().unwrap();
    info!("room = {}", room);
    let room_info = srv
        .get_ref()
        .send(GetRoomInfo(room))
        .await
        .expect("join room error!");
    info!("{:?}", room_info);
    if let Some(info) = room_info {
        Ok(HttpResponse::Ok().json(ResultJson::ok(info)))
    } else {
        Ok(HttpResponse::Ok().json(ResultJson::err(500, "未找到房间")))
    }
}
