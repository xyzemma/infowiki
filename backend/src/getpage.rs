use actix_web::{web::Path,Responder,HttpResponse};
pub fn getpagefn(path: Path<String>) -> impl Responder {
    let name = path.into_inner();
    let namepath: String = String::from(format!("pages/{name}/{name}html.html"));
    let namepath = namepath.as_str();
    if std::path::Path::new(namepath).exists() {
        let html = String::from(std::fs::read_to_string(namepath).unwrap());
        HttpResponse::Ok().body(format!("{}",html))
    }
    else {
        HttpResponse::Ok().body("Page not found")
    }
}