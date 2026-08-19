mod models;

use models::jwt::Claims;
use models::user::User;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web, web::Json};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct SignupInput {
    pub username: String,
    pub password: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/signup")]
async fn sign_up(req_body: Json<SignupInput>, app_state: web::Data<AppState>) -> impl Responder {
    let SignupInput { username, password } = req_body.into_inner();
    println!("{}", username);
    println!("{}", password);

    let id = {
        let mut user_index = app_state.user_index.lock().unwrap();
        let id = *user_index;
        *user_index += 1;
        id
    };
    let claims = {
        let mut users = app_state.users.lock().unwrap();
        users.push(User {
            id,
            username: username.clone(),
            password,
            usd: 0,
            sol: 0,
            eth: 0,
        });
        println!("{}", users.len());
        Claims {
            id,
            username,
            exp: 1_800_000_000,
        }
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"my-secret"),
    );

    HttpResponse::Ok().body("Done")
}

struct AppState {
    user_index: Mutex<u32>,
    users: Mutex<Vec<User>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users = web::Data::new(AppState {
        user_index: Mutex::new(0),
        users: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
            .service(hello)
            .service(sign_up)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
