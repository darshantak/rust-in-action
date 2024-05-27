use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize,Deserialize};

#[derive(Serialize)]
struct Message{
    content:String
}

async fn index()-> impl Responder{
    // "Hello from the backend"
let message = Message{
    content: "Hello from the server".to_string()
};
HttpResponse::Ok().json(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    HttpServer::new(|| {
        App::new().route("/api/message", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run().await
}

