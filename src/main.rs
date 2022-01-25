mod dbHelper;
use postgres::{Client, Error, NoTls};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde_json::json;
use serde::{Deserialize, Serialize};
#[get("/")]
async fn Get() -> impl Responder { 
    let x = dbHelper::Get().expect("error") ;
    //let str = String::from(&x[0].Text);
    HttpResponse::Ok().json(&x[0..])
   // HttpResponse::Ok().json("Welcome to QnA!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Get)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}