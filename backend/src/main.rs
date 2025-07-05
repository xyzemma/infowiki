use actix_web::{  get, post, web::Json, App, HttpResponse, HttpServer, Responder };
use serde_derive::Deserialize;
use actix_cors::Cors;
mod createpage;


#[actix_web::main]
async fn main() {
    if std::path::Path::new("pages").exists() != true {
        match std::fs::create_dir_all(format!("pages")) {
            Ok(_) => {}
            Err(error) => {
                println!("{}",error)
            }
        }
    }
    HttpServer::new( move ||  {
        let cors = Cors::default()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .service(hello)
            .service(getpage)
    })
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .await
        .unwrap()
}

#[post("/createpage")]
async fn hello(info : Json<Info>) -> impl Responder {
    let success: createpage::CrpResp = createpage::create_page(info.name.clone(), info.text.clone());
    let mut msg = String::new();
    match success {
        createpage::CrpResp::Success => {
            msg = format!("Succesfully created page {}",info.name);
        }
        createpage::CrpResp::Error(error) => {
            msg = error;
        }
    }
    
    HttpResponse::Ok().body(msg)
}

#[get("/wiki/{name}")]
async fn getpage(name:String) -> impl Responder {
    HttpResponse::Ok().body("test")
}

#[derive(Deserialize)]
struct Info {
    name: String,
    text: String
}