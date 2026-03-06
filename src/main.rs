use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use serde::Serialize;
use serde_json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/echo_teste")]
async fn echo_teste() -> impl Responder {
    HttpResponse::Ok().body("Teste 3!")
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    // HttpResponse::Ok().body("Hey there!")
    #[derive(Serialize, Debug)] // Derive Serialize trait for a custom struct
    struct User {
        name: String,
        age: u8,
    }

    let user = User {
        name: "John Doe".to_string(),
        age: 30,
    };
    // Convert the struct into a JSON string
    let json_string = serde_json::to_string(&user).unwrap();

    HttpResponse::Ok().body(json_string)
    // HttpResponse::Ok().body("Hey There!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(echo_teste)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}