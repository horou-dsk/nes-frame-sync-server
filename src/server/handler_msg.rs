use actix::{Handler, Context};
use crate::server::messages::{Connect, Disconnect};
use crate::server::OnLineServer;
use rand::Rng;

impl Handler<Connect> for OnLineServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<u32>() as usize;
        self.sessions.insert(id, msg.addr);

        id
    }
}

impl Handler<Disconnect> for OnLineServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}
