use actix_web::{get, post, web,web::Json, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize,Deserialize)]
struct SignupInput {
    pub username : String,
    pub password : String
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/signup")]
async fn sign_up(req_body: Json<SignupInput>,app_state: web::Data<AppState>) -> impl Responder {
    println!("{}",req_body.username);
    println!("{}",req_body.password);
    let users = app_state.users.lock().unwrap();
    println!("{}",users.len());
    HttpResponse::Ok().body("req_body")
}

struct User {
    id: u32,
    username : String,
    password : String
}

struct AppState {
    users : Mutex<Vec<User>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users = web::Data::new(AppState{users: Mutex::new(vec![User{id:2,username:String::from("shivam"),password:String::from("safsaef")}])});
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