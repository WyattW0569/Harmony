use actix::prelude::{Message, Recipient};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

// This is what WebSocketConnection is looking for
#[derive(Message)]
#[rtype(result = "()")] // String like we defined in response handler
pub struct WsMessage(pub String);

// WebSocketConnection will send this to a lobby to say "Connect me to lobby"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
}

// WebSocketConnection will send this to a lobby to say "Disconnect me from lobby"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub id: Uuid,
}

// WebSocketConnection will send this to a lobby for the lobby to echo out to everyone
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid
}

#[derive(Message)]
#[rtype(result = "HashMap<Uuid, HashSet<Uuid>>")] // try hashmap
pub struct GetRoomsMessage;

#[derive(Message)]
#[rtype(result = "HashMap<String, String>")] // try hashmap
pub struct GetNicksMessage;