use actix::{Recipient};
use actix::prelude::*;
use bytes::{Bytes};
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "()")]
pub enum Message {
    Text(String),
    Binary(Bytes)
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub room_id: u16,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub room_id: u16,
}

#[derive(Message)]
#[rtype(u16)]
pub struct CreateRoom(pub(crate) String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct KeyFrame {
    pub room_id: u16,
    pub frame: Bytes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomInfo {
    pub player: u8,
    pub game: String,
    pub room_id: u16,
}

#[derive(Message)]
#[rtype(result = "Option<RoomInfo>")]
pub struct GetRoomInfo(pub u16);
