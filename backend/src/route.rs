use crate::ws::WebSocketConnection;
use crate::lobby::Lobby;

use actix::Addr;
<<<<<<< HEAD
use actix_files::Files;
=======
>>>>>>> 7763ccee116f3ae32850e28a92a6ed1661992daa
use actix_web::{
    get,
    web::Data,
    web::Path,
    web::Payload,
    Error,
    HttpResponse, 
    HttpRequest,  
    web::Json,
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
<<<<<<< HEAD

    Ok(resp)
}
=======
    Ok(resp)
}
>>>>>>> 7763ccee116f3ae32850e28a92a6ed1661992daa
