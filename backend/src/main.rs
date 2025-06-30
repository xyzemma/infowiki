use actix_web::{  post, web::Json, App, HttpResponse, HttpServer, Responder };
use serde::Deserialize;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;



#[actix_web::main]
async fn main() {
    HttpServer::new( move ||  {
        App::new()
            .service(hello)
    })
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .await
        .unwrap()
}

#[post("/user")]
async fn hello(info : Json<Info>) -> impl Responder {
    let mut file = File::create(format!("{}.txt",info.name));
    let text: String = format!("{}",info.age);
    file.expect("REASON").write_all(text.as_bytes());
    let msg = format!("name: {}, age: {}", info.name, info.age);
    HttpResponse::Ok().body(msg)
}

#[derive(Deserialize)]
struct Info {
    name: String,
    age: i32
}