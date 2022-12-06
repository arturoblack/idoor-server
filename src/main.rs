use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
//use serde::{Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CustomResponse {
    status: String,
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[get("/hodor")]
async fn hodor() -> web::Json<CustomResponse> {
    web::Json(CustomResponse{ 
        status: String::from("500"), 
        message: String::from("hola")
    })
}

#[get("/hodor2")]
async fn hodor2(req: HttpRequest) -> impl Responder {

    let basic_auth_header = req.headers().get("Authorization");
    let token: &str = basic_auth_header.unwrap().to_str().unwrap();
    return HttpResponse::Ok().json(
        web::Json(CustomResponse{ 
            status: String::from("500"), 
            message: format!("hola {}", token)
        })
    );
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

/*
fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("content-type")?.to_str().ok()
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(hodor)
            .service(hodor2)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await
}