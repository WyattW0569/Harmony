use Harmony::Testing;
use actix_web::{get,web,App,HttpResponse,HttpServer,Responder};


#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body(format!("Your number is : {}",Testing::new().number))
}

#[get("/tom")]
async fn tom() -> impl Responder{
    HttpResponse::Ok().body(format!("Welcome to the tom page!!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new()
        .service(hello)
        .service(tom))
        .bind("0.0.0.0:5050")?
        .run()
        .await 
}
