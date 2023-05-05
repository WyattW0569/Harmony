use actix::{StreamHandler, Actor}; 
use actix_web::{web, App, Error, HttpResponse, HttpRequest, HttpServer};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// websockets will live in rooms
// only websockets in the same room can communicate
pub struct WebSocketConnection {
    //Uuid: A random 128 bit value (Ex) 67e55044-10b1-426f-9247-bb680e5fe0c8
    room: Uuid,
}

pub struct MyWebSocket;


//each websocket will be an actor

impl Actor for MyWebSocket{
    //Actor has four methods. "started", "running", "stopping", "Stopped"
    fn started() {

    }

    fn stopping(){ 
    }
}

