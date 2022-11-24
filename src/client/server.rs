use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

use crate::CaptchaChallenge;

async fn healthz() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener, _cc: CaptchaChallenge) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || App::new().route("/healthz", web::get().to(healthz)))
        .listen(listener)?
        .run();
    Ok(server)
}
