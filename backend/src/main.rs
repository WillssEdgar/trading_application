use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::{Context, Tera};

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let rendered = tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn fetch_data() -> impl Responder {
    // This is the response to the HTMX request
    HttpResponse::Ok().body("<div id='content'>Here is your dynamic data!</div>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .route("/fetch-data", web::get().to(fetch_data)) // HTMX route
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
