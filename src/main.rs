use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// This struct represents state
struct AppState {
    app_name: String,
}


#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    let _name = format!("Hello {app_name}!") ;//
    HttpResponse::Ok().body("Hello Sachin, Welcome to {_name}!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080; 
    println!("Server started on port:: {:?}", port);
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service( web::scope("/app").route("/index.html", web::get().to(index)),)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}


