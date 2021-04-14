use actix::{Handler, Context, Recipient};
use crate::server::messages::{Connect, Disconnect, CreateRoom, KeyFrame, Message, GetRoomInfo, RoomInfo};
use crate::server::{OnLineServer, Room};
use rand::Rng;
use crate::server::frames_sync::FrameMessage;
use actix::dev::MessageResponse;
use crate::server::socket_message::SendParcel;
use log::{info};

impl Handler<Connect> for OnLineServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<u32>() as usize;
        self.sessions.insert(id, msg.addr);
        match self.rooms.get_mut(&msg.room_id) {
            Some(room) => {
                room.act.push(id);
                if room.act.len() == 2 {
                    let ready = serde_json::to_string(&SendParcel::GameReady).unwrap();
                    let mut addrs: Vec<Recipient<Message>> = Vec::new();
                    for id in &room.act {
                        match self.sessions.get(id) {
                            Some(addr) => {
                                addr.do_send(Message::Text(ready.clone())).unwrap();
                                addrs.push((*addr).clone())
                            }
                            None => {}
                        }
                    }
                    room.frames_sync.start(addrs)
                }
            }
            None => {}
        }
        info!("{:?}", self.rooms);
        id
    }
}

impl Handler<Disconnect> for OnLineServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<CreateRoom> for OnLineServer {
    type Result = u16;

    fn handle(&mut self, msg: CreateRoom, _: &mut Self::Context) -> Self::Result {
        let room_id = self.rng.gen::<u16>();
        let room = Room::new(msg.0);
        self.rooms.insert(room_id, room);
        info!("{:?}", self.rooms);
        room_id
    }
}

impl Handler<KeyFrame> for OnLineServer {
    type Result = ();

    fn handle(&mut self, msg: KeyFrame, _: &mut Self::Context) -> Self::Result {
        match self.rooms.get(&msg.room_id) {
            Some(room) => {
                room.frames_sync.send(FrameMessage::KeyBuffer(msg.frame))
            }
            None => {}
        }
    }
}

impl Handler<GetRoomInfo> for OnLineServer {
    type Result = Option<RoomInfo>;

    fn handle(&mut self, msg: GetRoomInfo, _: &mut Self::Context) -> Self::Result {
        info!("{:?}", self.rooms);
        match self.rooms.get(&msg.0) {
            Some(room) => {
                Some(RoomInfo {
                    room_id: msg.0,
                    player: room.act.len() as u8 + 1,
                    game: room.game.clone(),
                })
            }
            None => None
        }
    }
}
