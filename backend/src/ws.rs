use actix::{fut, ActorContext};
use actix::{Actor, Addr, Running, StreamHandler, WrapFuture, ActorFuture, ContextFutureSpawner};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;

use std::time::{Duration, Instant};

use uuid::Uuid; 

use crate::messages::{Disconnect, Connect, WsMessage, ClientActorMessage};
use crate::lobby::Lobby;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// websockets will live in rooms
// only websockets in the same room can communicate
pub struct WebSocketConnection {
    //Uuid: A random 128 bit value (Ex) 67e55044-10b1-426f-9247-bb680e5fe0c8
    room: Uuid,
    //Sockets or Actors stay in one place, the Addr is passed around and used to send and recieve. ex(self.addr.send_to("HELLO"))
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    //personal id's for private messaging
    id: Uuid,
}

impl WebSocketConnection {
    pub fn new(room: Uuid, lobby: Addr<Lobby>) -> WebSocketConnection{
        WebSocketConnection {
            id: Uuid::new_v4(),
            room,
            lobby_addr: lobby,
            hb: Instant::now(),
        }
    }
}

impl WebSocketConnection {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }
}

//each websocket will be an actor

impl Actor for WebSocketConnection{
    type Context = ws::WebsocketContext<Self>;

    //Actor has four methods. "started", "running", "stopping", "Stopped"
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect { // defined in messages.rs
                addr: addr.recipient(),
                lobby_id: self.room,
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    // if something goes wrong just stop the actor
                    _ => ctx.stop(),
                }
                // using send instead of do_send this is a future and needs to be awaited
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _:&mut Self::Context, ) -> Running{ 
        self.lobby_addr.do_send(Disconnect {id: self.id, room_id: self.room });// will be defined later in messages crate. Struct Disconnect
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketConnection{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // 
        // This function is pretty simple, just check what kind of message we are getting. (messages.rs)
        //
        match msg {
            // Keep hearbeat alive, Ping responds with a pong
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            },
            // reset clock, still alive
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            },
            // binaray, should never be called, send to socket context to deal with it
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin), 
            // if it's a close message. close.
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            // Not going implenet Coninuation, Basically just for messages that are too long for a single message
            Ok(ws::Message::Continuation(bin)) => ctx.stop(),
            // no operation
            Ok(ws::Message::Nop) => (),
            // Text message, what we want, send it to the lobby, lobby will send it out.
            Ok(Text(s)) => self.lobby_addr.do_send(ClientActorMessage {
                id: self.id,
                msg: s.to_string(),
                room_id: self.room,
            }),

             // Websocket Error
            Err(e) => panic!("{:?}",e),
        }
    }
}

// responding to message
// impl Handler<MAILTYPE> for ACTOR
impl Handler<WsMessage> for WebSocketConnection { //WsMessage defined in messages.rs
    type Result = (); // setting expected result type, must match type defined in messages.rs, can be changed

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0)
    }   
}

