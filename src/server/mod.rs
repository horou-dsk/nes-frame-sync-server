use std::collections::HashMap;
use actix::{Recipient, Actor, Context};
use crate::server::messages::Message;
use rand::prelude::ThreadRng;

pub mod messages;
mod handler_msg;
mod frames_sync;

pub struct OnLineServer {
    rng: ThreadRng,
    sessions: HashMap<usize, Recipient<Message>>, // 房间长连接
}

impl Actor for OnLineServer {
    type Context = Context<Self>;
}
