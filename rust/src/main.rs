mod models;

use models::jwt::{AssetBalanceType, Claims, SignupReturnType};
use models::user::{TokenType, User};

use actix_web::{
    App, HttpMessage, HttpResponse, HttpServer, Responder,
    dev::{Service, ServiceResponse},
    get,
    http::header,
    post, web,
    web::Json,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct SignupInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct Asset {
    pub asset: TokenType,
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
async fn balance_asset(
    claims: web::ReqData<Claims>,
    asset: web::Query<Asset>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let Asset { asset } = asset.into_inner();
    println!("{:?}", asset);

    let user_id = claims.id;

    let balance = {
        let users = app_state.users.lock().unwrap();

        let user = match users.iter().find(|x| x.id == user_id) {
            Some(user) => user,
            None => {
                return HttpResponse::NotFound().body("User not found");
            }
        };

        let balance = user.get_balance(&asset);
        balance
    };

    HttpResponse::Ok().json(AssetBalanceType {
        asset: balance,
        success: String::from("Balance retrieved"),
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
            .service(sign_up)
            .service(sign_in)
            .service(
                web::scope("")
                    .wrap_fn(|req, srv| {
                        let secret = std::env::var("JWT_SECRET")
                            .unwrap_or_else(|_| String::from("2025_secret"));

                        let auth_header = req
                            .headers()
                            .get(header::AUTHORIZATION)
                            .and_then(|h| h.to_str().ok());

                        let token_str = match auth_header.and_then(|h| h.strip_prefix("Bearer ")) {
                            Some(token) => token,
                            None => {
                                let res = HttpResponse::Unauthorized()
                                    .body("Missing or invalid Authorization header");
                                let srv_res = ServiceResponse::new(req.clone(), res);
                                return Box::pin(async move { Ok(srv_res) })
                                    as LocalBoxFuture<'static, Result<_, _>>;
                            }
                        };

                        let token_data = match decode::<Claims>(
                            token_str,
                            &DecodingKey::from_secret(secret.as_bytes()),
                            &Validation::default(),
                        ) {
                            Ok(data) => data,
                            Err(_) => {
                                let (req, _payload) = req.into_parts();
                                let res =
                                    HttpResponse::Unauthorized().body("Invalid or expired token");
                                return Box::pin(async move { Ok(req.into_response(res)) })
                                    as LocalBoxFuture<'static, Result<_, _>>;
                            }
                        };

                        // Store decoded claims in request extensions
                        req.extensions_mut().insert(token_data.claims);

                        let fut = srv.call(req);
                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    })
                    .service(balance_asset), // Add any other protected services here
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
