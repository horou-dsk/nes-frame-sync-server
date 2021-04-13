mod params;

use actix_web::{HttpResponse, Error, web, get};
use actix::Addr;
use crate::server;
use crate::server::messages::CreateRoom;
use crate::router::nes::params::GameName;
use crate::router::ResultJson;

type HttpResultResp = Result<HttpResponse, Error>;

#[get("/create_room")]
pub async fn create_room(info: web::Query<GameName>, srv: web::Data<Addr<server::OnLineServer>>) -> HttpResultResp {
    let game = info.0.game;
    let resp: u16 = srv.get_ref().send(CreateRoom(game)).await.expect("create room error!");
    Ok(HttpResponse::Ok().json(ResultJson::ok(resp)))
}
