use actix_web::{web,App,HttpServer,HttpResponse};
use serde::{Deserialize,Serialize};
use std::{fmt::format, sync::Mutex};

#[derive(Deserialize)]
struct GreetRequest{
    name:String,
}

#[derive(Serialize)]
struct GreetResponse{
    message:String,    
}

async fn greet(req:web::Json<GreetRequest>) -> HttpResponse{
    let response = GreetResponse{
        message: format!("Hello, {}",req.name)
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .route("/api/greet", web::post().to(greet))
        .service(actix_files::Files::new("/","./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?.run().await
   
}
