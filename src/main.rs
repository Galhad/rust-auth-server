use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};

async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(index))
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/index.html")
                    .finish()
            })))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
