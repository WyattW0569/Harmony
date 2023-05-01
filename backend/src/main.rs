use actix_files::Files;
use actix_web::{App,HttpServer};

mod api;

use api::route::{
    tom,
};


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new()
        .service(tom)
        .service(Files::new("/", "././frontend/dist/").index_file("index.html")))
        .bind("0.0.0.0:5050")?
        .run()
        .await 
}
//I wrote this comment at home!!!!