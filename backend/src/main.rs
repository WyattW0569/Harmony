use actix::{Actor, Addr};
use actix_web::{App,HttpServer, web::scope, web};
use once_cell::sync::Lazy;
use actix_cors::Cors;

mod route;
mod lobby;
mod ws;
mod messages;

use lobby::Lobby;

use route::{
    tom,
    start_connection,
    parse_rooms,
    parse_nicks,
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
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .allowed_methods(vec!["Content-Type"]);

        App::new()
            .wrap(cors)
            .service(
                scope("/api")
                    .service(tom)
                    .service(parse_rooms)
                    .service(parse_nicks)
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