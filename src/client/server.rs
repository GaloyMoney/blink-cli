use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use graphql_client::reqwest::post_graphql;
use reqwest::Client;

use std::net::TcpListener;

use crate::{queries::*, CaptchaChallenge, GaloyCliError};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

struct AppData {
    tera: Tera,
    phone: String,
    api: String,
}

async fn login(appdata: web::Data<AppData>) -> impl Responder {
    println!("Fetching Captcha Challenge...");
    let client = Client::builder().build().expect("Can't build client");
    let variables = captcha_create_challenge::Variables;
    let response =
        post_graphql::<CaptchaCreateChallenge, _>(&client, appdata.api.clone(), variables)
            .await
            .expect("Couldn't create Captcha");
    let response = response.data.expect("Captcha Data is missing");
    let captcha_challenge_result =
        CaptchaChallenge::try_from(response).expect("Couldn't parse Captcha Response Body");

    let mut ctx = Context::new();

    ctx.insert("id", &captcha_challenge_result.id);
    ctx.insert("new_captcha", &captcha_challenge_result.new_captcha);
    ctx.insert("failback_mode", &captcha_challenge_result.failback_mode);
    ctx.insert("challenge_code", &captcha_challenge_result.challenge_code);

    let rendered = appdata.tera.render("login.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Debug, Serialize, Deserialize)]
struct GeetestResponse {
    geetest_challenge: String,
    geetest_seccode: String,
    geetest_validate: String,
}

async fn solve(r: web::Json<GeetestResponse>, appdata: web::Data<AppData>) -> impl Responder {
    println!("Captcha Solved, you may close the browser and return to the CLI.");

    let client = Client::builder().build().expect("Can't build client");

    let input = captcha_request_auth_code::CaptchaRequestAuthCodeInput {
        challenge_code: r.geetest_challenge.clone(),
        phone: appdata.phone.clone(),
        sec_code: r.geetest_seccode.clone(),
        validation_code: r.geetest_validate.clone(),
    };
    let variables = captcha_request_auth_code::Variables { input };

    let response_body =
        post_graphql::<CaptchaRequestAuthCode, _>(&client, appdata.api.clone(), variables).await;

    match response_body {
        Ok(_) => println!("Phone Code sent successfully to {}", appdata.phone),
        Err(e) => {
            log::error!("{:?}", e);
            println!("Phone Code couldn't be send.")
        }
    };

    tokio::spawn(async {
        std::process::exit(0);
    });

    HttpResponse::Ok()
}

pub fn run(listener: TcpListener, phone: String, api: String) -> Result<Server, GaloyCliError> {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/public/**/*")).unwrap();

    let appdata = web::Data::new(AppData { tera, phone, api });

    let server = HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(ResourceFiles::new("/static", generated))
            .route("/login", web::get().to(login))
            .route("/solve", web::post().to(solve))
            .app_data(appdata.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
