use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod app_service {
    #[get("/")]
    pub async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!");
    }

    #[post("/echo")]
    pub async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req.req_body);
    }

    pub async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!");
    }
}