use std::time::{Instant, Duration};
use crate::server;
use actix::{Actor, ActorContext, AsyncContext, Addr, WrapFuture, ActorFutureExt, StreamHandler, Handler};
use actix::prelude::*;
use actix_web_actors::ws;
use crate::server::messages::{Connect, Message as ServerMessage, Disconnect, KeyFrame};
use actix_http::ws::{Message, ProtocolError};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


pub struct OnLineWebSocket {
    pub id: usize,
    pub hb: Instant,
    pub addr: Addr<server::OnLineServer>,
    pub room_id: u16,
}

impl OnLineWebSocket {
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // act.addr.do_send(server::Disconnect { id: act.id });
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for OnLineWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr.send(Connect {
            addr: addr.recipient(),
            room_id: self.room_id,
        })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(id) => act.id = id,
                    _ => ctx.stop()
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect { id: self.id, room_id: self.room_id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for OnLineWebSocket {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(_text) => {

            }
            ws::Message::Binary(bytes) => {
                self.addr.do_send(KeyFrame {
                    room_id: self.room_id,
                    frame: bytes,
                })
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl Handler<ServerMessage> for OnLineWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ServerMessage::Text(text) => {
                ctx.text(text)
            },
            ServerMessage::Binary(bytes) => {
                ctx.binary(bytes)
            }
        }
    }
}
