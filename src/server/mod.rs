use std::collections::HashMap;
use actix::{Recipient, Actor, Context};
use crate::server::messages::Message;
use rand::prelude::ThreadRng;
use crate::server::frames_sync::Frames;

pub mod messages;
mod handler_msg;
mod frames_sync;
mod socket_message;

#[derive(Debug)]
pub struct Room {
    pub act: Vec<usize>,
    pub frames_sync: Frames,
    pub game: String,
}

impl Room {
    pub fn new(game: String) -> Self {
        Self {
            act: Vec::new(),
            frames_sync: Frames::new(),
            game,
        }
    }
}

pub struct OnLineServer {
    rng: ThreadRng,
    sessions: HashMap<usize, Recipient<Message>>, // 房间长连接
    rooms: HashMap<u16, Room>
}

impl Actor for OnLineServer {
    type Context = Context<Self>;
}

impl OnLineServer {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    // pub fn send_message(&self) {
    //     self.sessions[&1].do_send(Message::Text("我日".into()));
    // }
}
