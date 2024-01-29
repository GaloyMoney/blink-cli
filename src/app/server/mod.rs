use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use graphql_client::reqwest::post_graphql;
use reqwest::Client;

use std::net::TcpListener;

use crate::client::queries::{captcha_request_auth_code, CaptchaChallenge, CaptchaRequestAuthCode};

struct AppData {
    tera: Tera,
    phone: String,
    api: String,
    captcha_challenge_result: CaptchaChallenge,
}

#[actix_web::get("/login")]
async fn login(appdata: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();

    ctx.insert("id", &appdata.captcha_challenge_result.id);
    ctx.insert("new_captcha", &appdata.captcha_challenge_result.new_captcha);
    ctx.insert(
        "failback_mode",
        &appdata.captcha_challenge_result.failback_mode,
    );
    ctx.insert(
        "challenge_code",
        &appdata.captcha_challenge_result.challenge_code,
    );

    let rendered = appdata.tera.render("login.tera.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Debug, Serialize, Deserialize)]
struct GeetestResponse {
    geetest_challenge: String,
    geetest_seccode: String,
    geetest_validate: String,
}

#[actix_web::post("/solve")]
async fn solve(r: web::Json<GeetestResponse>, appdata: web::Data<AppData>) -> impl Responder {
    println!("Captcha Solved, you may close the browser and return to the CLI.");

    let client = Client::builder().build().expect("Can't build client");

    let input = captcha_request_auth_code::CaptchaRequestAuthCodeInput {
        challenge_code: r.geetest_challenge.clone(),
        phone: appdata.phone.clone(),
        sec_code: r.geetest_seccode.clone(),
        validation_code: r.geetest_validate.clone(),
        channel: None,
    };
    let variables = captcha_request_auth_code::Variables { input };

    let response_body =
        post_graphql::<CaptchaRequestAuthCode, _>(&client, appdata.api.clone(), variables).await;

    match response_body {
        Ok(_) => println!("Phone Code sent successfully to {}", appdata.phone),
        Err(_) => {
            println!("Phone Code couldn't be send.")
        }
    };

    tokio::spawn(async {
        std::process::exit(0);
    });

    HttpResponse::Ok()
}

#[derive(RustEmbed)]
#[folder = "src/app/server/public/"]
struct Asset;

#[actix_web::get("/static/{_:.*}")]
async fn static_dir(path: web::Path<String>) -> impl Responder {
    match Asset::get(&path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path.as_str()).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub async fn run(
    listener: TcpListener,
    phone: String,
    api: String,
    captcha_challenge_result: CaptchaChallenge,
) -> anyhow::Result<()> {
    let mut tera = Tera::default();
    tera.add_raw_template("login.tera.html", include_str!("./public/login.tera.html"))?;

    let appdata = web::Data::new(AppData {
        tera,
        phone,
        api,
        captcha_challenge_result,
    });

    let server = HttpServer::new(move || {
        App::new()
            .service(static_dir)
            .service(login)
            .service(solve)
            .app_data(appdata.clone())
    })
    .listen(listener)?
    .run();

    server.await?;
    Ok(())
}
