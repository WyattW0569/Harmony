use actix::Actor;
use actix_files::Files;
use actix_web::{App,HttpServer, web::scope};

mod route;
<<<<<<< HEAD
=======
use route::start_connection;
>>>>>>> 7763ccee116f3ae32850e28a92a6ed1661992daa
mod lobby;
use lobby::Lobby;
mod ws;
mod messages;

use route::{
    tom,
    start_connection,
};

<<<<<<< HEAD
//.service(Files::new("/", "././frontend/dist/").index_file("index.html"))

=======
>>>>>>> 7763ccee116f3ae32850e28a92a6ed1661992daa
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let chat_server = Lobby::default().start();
    HttpServer::new(move || {
        App::new()
<<<<<<< HEAD
            .service(
                scope("/api")
                    .service(tom)
                    .service(start_connection)
            )
            .data(chat_server.clone())
        })
        .bind(("0.0.0.0", 80))?
=======
            .service(tom)
            .service(start_connection)
            .service(Files::new("/", "././frontend/dist/").index_file("index.html"))
            .data(chat_server.clone())
        })
        .bind(("0.0.0.0", 8080))?
>>>>>>> 7763ccee116f3ae32850e28a92a6ed1661992daa
        .run()
        .await 
}
//I wrote this comment at home!!!!