mod handlers;

use actix_web::{web, App, HttpServer};




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    /*let key_listener = std::net::TcpListener::bind("0.0.0.0:353")?;

    for stream in key_listener.incoming() {
        let mut buf = [0;3000];
        println!("{:?}",stream?.peek(&mut buf));
        println!("{:?}",std::str::from_utf8(&buf));
    }*/

    HttpServer::new(move ||{
        App::new()
        .route("/create",web::post().to(handlers::create))
        .route("/info",web::post().to(handlers::info))
        .route("/update",web::get().to(handlers::update))
    })
        .bind("0.0.0.0:352")?
        .run()
        .await
}