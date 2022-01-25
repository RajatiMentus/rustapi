use postgres::{Client, Error, NoTls};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io;
use std::vec;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[derive(Debug)]
//#[derive(serde::Serialize)]
pub struct question {
    pub ID: i32,
    pub Text: String,
    //pub Option:String,
}

pub fn Get() -> Result<Vec<question>, Error> {
    println!("Connecting DB");
    let mut client = Client::connect("postgresql://postgres:rajat123@localhost/Questions_db", NoTls)?;

    let id: u32 = 1;
    println!("Connection Successful");
    let mut arr:Vec<question> = Vec::new();
    for row in client.query("SELECT \"ID\", \"Text\" FROM public.\"Question\";", &[])? {           
        arr.push(question{
            ID: row.get(0),
            Text: row.get(1),
            //Option: row.get(2)
        });
        //println!("Question {} is {}", question.ID, question.Text);
    }
    Ok(arr)
    //Ok(HttpResponse::Ok().json(arr))
}