use actix_files as fs;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use tera::{Context, Tera};

use std::net::TcpListener;

use crate::CaptchaChallenge;

async fn login(cc: web::Data<CaptchaChallenge>, tera: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();

    ctx.insert("id", &cc.id);
    ctx.insert("new_captcha", &cc.new_captcha);
    ctx.insert("failback_mode", &cc.failback_mode);
    ctx.insert("challenge_code", &cc.challenge_code);

    let rendered = tera.render("login.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub fn run(listener: TcpListener, cc: CaptchaChallenge) -> Result<Server, std::io::Error> {
    let cc_appdata = web::Data::new(cc);
    let tera_appdata =
        web::Data::new(Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/public/**/*")).unwrap());
    let server = HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "src/public/").show_files_listing())
            .route("/login", web::get().to(login))
            .app_data(cc_appdata.clone())
            .app_data(tera_appdata.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
