use actix_web::{  post, web::Json, App, HttpResponse, HttpServer, Responder };
use serde::Deserialize;
use serde_derive::Deserialize;
use actix_cors::Cors;
mod createpage;


#[actix_web::main]
async fn main() {
    HttpServer::new( move ||  {
        let cors = Cors::default()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .service(hello)
    })
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .await
        .unwrap()
}

#[post("/createpage")]
async fn hello(info : Json<Info>) -> impl Responder {
    let success: bool = createpage::create_page(info.name.clone(), info.text.clone());
    let mut msg = String::new();
    if success == true {
        msg = format!("Created Page: {}",info.name);
    } else {
        msg = format!("Failed to create Page: {}",info.name)
    }
    
    HttpResponse::Ok().body(msg)
}

#[derive(Deserialize)]
struct Info {
    name: String,
    text: String
}