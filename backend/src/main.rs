use actix_web::{  get, post, web::Json,web::Path, App, HttpResponse, HttpServer, Responder };
use serde_derive::Deserialize;
use actix_cors::Cors;
use crate::getpage::getpagefn;
mod init;
mod createpage;
mod getpage;
mod parse;
mod updatepage;
mod versioncontrol;
mod users;

#[actix_web::main]
async fn main() {
    let is_debug: bool = match std::env::var("DEBUG") {
        Ok(val) => {
            if val == String::from("true") {
                true
            } else {
                false
            }
        },
        Err(e) => {panic!("{e}")}
    };
    let (maindir,pagedir) = init::init().await;
    HttpServer::new( move ||  {
        let cors = Cors::default()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .service(createpagesrv)
            .service(getpagesrv)
    })
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .await
        .unwrap()
}

#[post("/createpage")]
async fn createpagesrv(info : Json<Info>) -> impl Responder {
    let success = createpage::create_page(info.name.clone(), info.text.clone());
    let mut msg = String::new();
    match success {
        init::IwResp::Success => {
            msg = format!("Succesfully created page {}",info.name);
        }
        init::IwResp::Error(error) => {
            msg = error;
        }
    }
    
    HttpResponse::Ok().body(msg)
}

#[get("/wiki/{name}")]
async fn getpagesrv(path: Path<String>) -> impl Responder {
    getpagefn(path)
}

#[derive(Deserialize)]
struct Info {
    name: String,
    text: String
}