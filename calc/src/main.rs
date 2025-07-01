//web microservice for calculator multiple types of operations
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Calculator API!")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::add(info.0, info.1);
    HttpResponse::Ok().body(format!("Result of {} + {} = {}", info.0, info.1, result))
}

#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::subtract(info.0, info.1);
    HttpResponse::Ok().body(format!("Result of {} - {} = {}", info.0, info.1, result))
}

#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::multiply(info.0, info.1);
    HttpResponse::Ok().body(format!("Result of {} * {} = {}", info.0, info.1, result))
}

#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    match calc::divide(info.0, info.1) {
        Ok(result) => {
            HttpResponse::Ok().body(format!("Result of {} / {} = {}", info.0, info.1, result))
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
