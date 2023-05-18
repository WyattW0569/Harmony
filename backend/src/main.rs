use actix::{Actor, Addr};
use actix_files::Files;
use actix_web::{App,HttpServer, web::scope, web};
use once_cell::sync::Lazy;

mod route;
mod lobby;
use lobby::Lobby;
mod ws;
mod messages;

use route::{
    tom,
    start_connection,
    parse_rooms,
};

//.service(Files::new("/", "././frontend/dist/").index_file("index.html"))

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    //Lobby::default().start();
    static LOBBY: Lazy<Addr<Lobby>> = Lazy::new(||{ 
        let lobby = Lobby::default().start();
        lobby
    });

    HttpServer::new(move || {
        App::new()
            .service(
                scope("/api")
                    .service(tom)
                    .service(parse_rooms)
                    .service(start_connection)
            )
            .app_data(web::Data::new(LOBBY.clone()))
            .data(LOBBY.clone())
        })
        .bind(("0.0.0.0", 80))?
        .run()
        .await 
}
//I wrote this comment at home!!!!