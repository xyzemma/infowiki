use actix_web::guard::Get;
use actix_web::{  get, post, web::Json,web::Path, App, HttpResponse, HttpServer, Responder };
use serde_derive::Deserialize;
use actix_cors::Cors;
use std::fs::{remove_file,remove_dir_all};
use std::process;
use crate::getpage::getpagefn;
mod init;
mod createpage;
mod getpage;
mod parse;
mod updatepage;
mod versioncontrol;
mod users;
mod errorhandling;

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
    let (maindir,pagedir,pid) = init::init().await;
    HttpServer::new( move ||  {
        let cors = Cors::default()
            .allow_any_origin();
        match is_debug {
            false => {
                App::new()
                .wrap(cors)
                .service(createpagesrv)
                .service(getpagesrv)
            }
            true => {
                App::new()
                .wrap(cors)
                .service(createpagesrv)
                .service(getpagesrv)
                .service(debugquitsrv)
            }
    }
})
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .await
        .unwrap()
}

#[post("/debugquit")]
async fn debugquitsrv() -> impl Responder {
    let (mainpath,pagepath) = match init::get_paths() {
        Ok(val) => val,
        Err(e) => {
            panic!("{}",e.errormsg)
        }
    };
    remove_dir_all(pagepath).unwrap();
    remove_file(format!("{}/db.db3",mainpath)).unwrap();
    process::exit(0);
    HttpResponse::Ok().body("Quitting...")
}
#[post("/createpage")]
async fn createpagesrv(info : Json<Info>) -> impl Responder {
    let success = createpage::create_page(info.name.clone(), info.text.clone());
    let mut msg = String::new();
    match success {
        errorhandling::IwResp::Success => {
            msg = format!("Succesfully created page {}",info.name);
        }
        errorhandling::IwResp::Error(error) => {
            msg = error.errormsg;
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