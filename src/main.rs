use actix_web::{App, HttpServer};

mod hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip: String = String::from("127.0.0.1:8080");
    println!("Running on: {}", ip);
    HttpServer::new(|| App::new().service(hello::get_hello).service(hello::echo))
        .bind(ip)?
        .run()
        .await
}