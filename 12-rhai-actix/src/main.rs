use actix_web::{
    get,        // create get handler
    web::path,  // extract path parameters
    App,        // create app
    HttpServer, // ceate server
    Responder,  // create response
};

#[actix_web::main]
fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(Multiply).service(Add))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await()
}
