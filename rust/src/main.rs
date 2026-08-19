mod models;

use models::jwt::{Claims, SignupReturnType};
use models::user::User;

use models::order::market;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web, web::Json};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::models::jwt::AssetBalanceType;

#[derive(Serialize, Deserialize)]
struct SignupInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct Asset {
    pub asset: market,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/signup")]
async fn sign_up(req_body: Json<SignupInput>, app_state: web::Data<AppState>) -> impl Responder {
    let SignupInput { username, password } = req_body.into_inner();
    let secret = std::env::var("JWT_SECRET").unwrap_or(String::from("2025_secret"));
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
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    HttpResponse::Ok().json(SignupReturnType {
        token,
        success: String::from("User Created successfully"),
    })
}

#[post("/signin")]
async fn sign_in(req_body: Json<SignupInput>, app_state: web::Data<AppState>) -> impl Responder {
    let SignupInput { username, password } = req_body.into_inner();
    let secret = std::env::var("JWT_SECRET").unwrap_or(String::from("2025_secret"));
    println!("{}", username);
    println!("{}", password);

    let id = {
        let users = app_state.users.lock().unwrap();

        match users
            .iter()
            .find(|x| x.username == username && x.password == password)
        {
            Some(user) => user.id,
            None => {
                return HttpResponse::Unauthorized().body("Invalid username or password");
            }
        }
    };

    let claims = {
        Claims {
            id,
            username,
            exp: 1_800_000_000,
        }
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    HttpResponse::Ok().json(SignupReturnType {
        token,
        success: String::from("Login successful"),
    })
}

#[get("/balance")]
async fn balance_asset(asset: web::Query<Asset>, app_state: web::Data<AppState>) -> impl Responder {
    let Asset { asset } = asset.into_inner();
    let secret = std::env::var("JWT_SECRET").unwrap_or(String::from("2025_secret"));
    println!("{:?}", asset);

    let balance = {
        let users = app_state.users.lock().unwrap();

        match users.iter().find(|x| x.id == id) {
            Some(user) => user[asset],
            None => {
                return HttpResponse::Unauthorized().body("Invalid username or password");
            }
        }
    };

    HttpResponse::Ok().json(AssetBalanceType {
        asset: 0,
        success: String::from("Login successful"),
    })
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
            .service(balance_asset)
            .service(sign_up)
            .service(sign_in)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
