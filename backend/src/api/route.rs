use actix_web::{
    get,
    web,
    HttpResponse,
    web::Json,
    web::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TomIdentifier {
    tom_id: String,
}

#[get("/tom/{tom_id}")]
pub async fn tom(tom_identifier: Path<TomIdentifier>) -> Json<String> {
    return Json(tom_identifier.into_inner().tom_id);
}
