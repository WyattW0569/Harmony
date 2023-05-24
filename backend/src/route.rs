use crate::ws::WebSocketConnection;
use crate::lobby::Lobby;
use crate::messages::GetRoomsMessage;

use std::collections::{HashMap, HashSet};

use actix::Addr;
use actix_files::Files;
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
pub async fn parse_rooms(lobby: web::Data<Addr<Lobby>>) -> Json<String> {
    let resp = lobby.send(GetRoomsMessage).await.unwrap();
    return Json(format!("{:?}",resp));
}