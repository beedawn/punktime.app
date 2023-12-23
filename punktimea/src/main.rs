use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_web::http::header;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().send_wildcard();


            // .allowed_origin("https://www.rust-lang.org")
            // .allowed_methods(vec!["GET", "POST"])
            // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            // .allowed_header(header::CONTENT_TYPE)
            // .max_age(3600);




        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
