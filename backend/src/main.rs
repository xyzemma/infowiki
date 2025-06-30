use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer,Responder,get,post};
use serde_derive::Deserialize;
use actix_cors::Cors;
use reqwest::header::USER_AGENT;
use reqwest::Client;

#[derive(Debug, Deserialize)]
pub struct Params {
    url: String,
}
#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
#[post("/createpage")]
async fn index(info: web::Json<Info>) -> String {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin();
        App::new().wrap(cors).service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}