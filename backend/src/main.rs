use actix::Actor;
use actix_files::Files;
use actix_web::{App,HttpServer, web::scope};

mod route;
mod lobby;
use lobby::Lobby;
mod ws;
mod messages;

use route::{
    tom,
    start_connection,
};

//.service(Files::new("/", "././frontend/dist/").index_file("index.html"))

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let chat_server = Lobby::default().start();
    HttpServer::new(move || {
        App::new()
            .service(
                scope("/api")
                    .service(tom)
                    .service(start_connection)
            )
            .data(chat_server.clone())
        })
        .bind(("0.0.0.0", 80))?
        .run()
        .await 
}
//I wrote this comment at home!!!!