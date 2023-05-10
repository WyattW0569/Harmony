use actix::Actor;
use actix_files::Files;
use actix_web::{App,HttpServer};

mod route;
use route::start_connection;
mod lobby;
use lobby::Lobby;
mod ws;
mod messages;

use route::{
    tom,
};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let chat_server = Lobby::default().start();
    HttpServer::new(move || {
        App::new()
            .service(tom)
            .service(start_connection)
            .service(Files::new("/", "././frontend/dist/").index_file("index.html"))
            .data(chat_server.clone())
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await 
}
//I wrote this comment at home!!!!