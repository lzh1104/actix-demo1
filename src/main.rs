use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use routes::nn;

mod routes;

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

async fn index() -> impl Responder {
    "Hello World!"
}


struct AppState {
    app_name: String,
}

#[get("/app_name_test")]
async fn app_name_test(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .app_data(web::Data::new(AppState {
            app_name: String::from("Actix Web Test1"),
        }))
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .service(
            web::scope("/app")
            // /app/index.html
            .route("/index.html", web::get().to(index))
            .route("/json1", web::get().to(nn::history_dates))
        )
        .service(app_name_test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
