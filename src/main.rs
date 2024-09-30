use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Greeting {
    message: String,
}

#[get("/")]
async fn greet() -> impl Responder {
    let greeting = Greeting {
        message: "Xin chào từ ứng dụng Rust!".to_string(),
    };
    HttpResponse::Ok().json(greeting)
}

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[post("/echo")]
async fn echo(info: web::Json<Info>) -> impl Responder {
    let response = format!("Xin chào, {}!", info.name);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(echo)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
