use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("<h1>test</h1>")
}

#[post("/echo")]
async fn echo(body: String) -> impl Responder {
    let mut response_template: String = String::from("Response is: ");
    response_template.push_str(&body);
    HttpResponse::Ok().body(response_template)
}
