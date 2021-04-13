use actix::{Handler, Context};
use crate::server::messages::{Connect, Disconnect, CreateRoom};
use crate::server::{OnLineServer, Room};
use rand::Rng;

impl Handler<Connect> for OnLineServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<u32>() as usize;
        // let addr = msg.addr.clone();
        self.sessions.insert(id, msg.addr);
        match self.rooms.get_mut(&msg.room_id) {
            Some(room) => {
                room.act.push(id);
            }
            None => {}
        }
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
        room_id
    }
}
