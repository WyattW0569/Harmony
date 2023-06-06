use crate::ws::WebSocketConnection;
use crate::lobby::Lobby;
use crate::messages::{GetRoomsMessage, GetNicksMessage};

use std::collections::{HashMap, HashSet};

use actix::Addr;
use actix_web::{
    get,
    web::Data,
    web::Path,
    web::Payload,
    Error,
    HttpResponse, 
    HttpRequest,  
    web::Json,
    web,
    Responder,
};
use actix_web_actors::ws;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TomIdentifier {
    tom_id: String,
}

#[get("/tom/{tom_id}")]
pub async fn tom(tom_identifier: Path<TomIdentifier>) -> Json<String> {
    return Json(tom_identifier.into_inner().tom_id);
}

#[get("/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    Path(group_id): Path<Uuid>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let ws = WebSocketConnection::new(
        group_id,
        srv.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;

    Ok(resp)
}

#[get("/rooms")]
pub async fn parse_rooms(lobby: web::Data<Addr<Lobby>>) -> impl Responder {
    let resp = lobby.send(GetRoomsMessage).await;
    let rooms = match resp {
        Ok(rooms) => rooms,
        Err(_) => {
            return HttpResponse::InternalServerError()
            .body("Failed to retrieve rooms");
        }
    };

    let converted_rooms: HashMap<String, HashSet<String>> = rooms
        .into_iter()
        .map(|(room_id, population_set)| {
            let converted_pop: HashSet<String> = population_set
                .into_iter()
                .map(|user_id| user_id.to_string())
                .collect();
                (room_id.to_string(), converted_pop)
        })
        .collect();
    
    HttpResponse::Ok().json(converted_rooms)
}

#[get("/nicks")]
pub async fn parse_nicks(lobby: web::Data<Addr<Lobby>>) -> impl Responder {
    let resp = lobby.send(GetNicksMessage).await;
    let nick_names = match resp {
        Ok(nicks) => nicks,
        Err(_) => {
            return HttpResponse::InternalServerError()
            .body("Failed to retrieve rooms");
        }
    };

    HttpResponse::Ok().json(nick_names)
}