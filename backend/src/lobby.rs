use crate::messages::{Connect, Disconnect, ClientActorMessage, WsMessage, GetRoomsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient, MessageResult};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

// Lobby is an actor, but actor is just a struct

type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>, // self id
    rooms: HashMap<Uuid, HashSet<Uuid>>, // list of users in a room
}

impl Default for Lobby{
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

// impl a method that actually sends a message to client
impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        // simply checks if Uuid of reciepend exists and sends string as a WsMessage if so, otherwise prints error.
        if let Some(socket_reciepient) = self.sessions.get(id_to) {
            let _ = socket_reciepient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("Can't find user id, unable to send message")
        }

    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

// Now Impl Handler for each Message type, Disconnect, Connect, ClientActorMessage

// Disconnect Handler. Either, removes client from lobby and sends everyone else disconnect message, or closes lobby if only you.
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| self.send_message(&format!("{} disconnected.", &msg.id), user_id));
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    // only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

// Connection Handler
impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // create a room if need, then add the id to it
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new).insert(msg.self_id);

        self.rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.self_id), conn_id));


        // store the address
        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );

        // send self your new uuid
        self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}

// handle ActorClientMessage, open lobby to read messages from clients and let lobby forward messages to clients
impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Context<Self>) -> Self::Result {
        // checking if message start with \w to whisper to specific client with Uuid
        // in the future see it this works with a match statement
        match msg.msg.split_whitespace().collect::<Vec<&str>>().get(2).expect("Check for Command") {
            whisper if whisper.starts_with("!w") => {
                if let Some(id_to) = msg.msg.split_whitespace().collect::<Vec<&str>>().get(3) {
                    if let Ok(uuid) = &Uuid::parse_str(id_to) {
                        self.send_message(&msg.msg, uuid);
                    } else {
                        println!("Invalid Whisper");
                    }
                }
            },
            help if help.starts_with("!help") => {
                println!("System Message: \n'!w [id] [message]' to whisper\n'!help' for help")
            },
            nickname if nickname.starts_with("!nick") => {
                if let Some(nick) = msg.msg.split_whitespace().collect::<Vec<&str>>().get(3) {
                    println!("Set NickName to - {}", nick);
                }
            },
            _ => self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| self.send_message(&msg.msg, client)),
        }

        /*if msg.msg.split_whitespace().collect::<Vec<&str>>().get(2).expect("Check message if Whisper").starts_with("!w") {
            // add a check for invalid ID's
            if let Some(id_to) = msg.msg.split_whitespace().collect::<Vec<&str>>().get(3) {
                if let Ok(uuid) = &Uuid::parse_str(id_to) {
                    self.send_message(&msg.msg, uuid);
                } else {
                    println!("Invalid Whisper");
                }
            }
        } else if msg.msg.split_whitespace().collect::<Vec<&str>>().get(2).expect("Check message if Help").starts_with("!help") {
            println!("System Message: \n'!w [id] [message]' to whisper\n'!help' for help")
        
        } else if msg.msg.split_whitespace().collect::<Vec<&str>>().get(2).expect("Check message if NickName").starts_with("!nick") {
            if let Some(nick) = msg.msg.split_whitespace().collect::<Vec<&str>>().get(3) {
                println!("Set NickName to - {}", nick)
            }
        }else {
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| self.send_message(&msg.msg, client));

        }*/
    }
}

impl Handler<GetRoomsMessage> for Lobby {
    type Result = MessageResult<GetRoomsMessage>;

    fn handle(&mut self, _: GetRoomsMessage, _:&mut Context<Self>) -> Self::Result {
        
        return MessageResult(self.rooms.clone());
        
    }
}