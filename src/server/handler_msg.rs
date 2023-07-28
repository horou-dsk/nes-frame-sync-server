use crate::server::frames_sync::FrameMessage;
use crate::server::messages::{
    Connect, CreateRoom, Disconnect, GetRoomInfo, KeyFrame, Message, RoomInfo,
};
use crate::server::socket_message::SendParcel;
use crate::server::{OnLineServer, Room};
use actix::{Context, Handler, Recipient};
use log::info;
use rand::Rng;

impl Handler<Connect> for OnLineServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<u32>() as usize;
        self.sessions.insert(id, msg.addr);
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            room.act.push(id);
            if room.act.len() == 2 {
                let ready = serde_json::to_string(&SendParcel::GameReady).unwrap();
                let mut addrs: Vec<Recipient<Message>> = Vec::new();
                for id in &room.act {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(Message::Text(ready.clone()));
                        addrs.push((*addr).clone())
                    }
                }
                room.frames_sync.start(addrs)
            }
        }
        info!("{:?}", self.rooms);
        id
    }
}

impl Handler<Disconnect> for OnLineServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            room.frames_sync.send(FrameMessage::Stop)
        }
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
        if let Some(room) = self.rooms.get_mut(&msg.room_id) {
            info!("{:?}", msg.frame.to_vec());
            room.frames_sync.send(FrameMessage::KeyBuffer(msg.frame))
        }
    }
}

impl Handler<GetRoomInfo> for OnLineServer {
    type Result = Option<RoomInfo>;

    fn handle(&mut self, msg: GetRoomInfo, _: &mut Self::Context) -> Self::Result {
        info!("{:?}", self.rooms);
        self.rooms.get(&msg.0).map(|room| RoomInfo {
            room_id: msg.0,
            player: room.act.len() as u8 + 1,
            game: room.game.clone(),
        })
    }
}
