#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate rocket;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


use rocket::{Request, Route, Data, Catcher, Error};
use rocket::http::Method::*;
use rocket::handler::Outcome;


#[derive(Serialize, Deserialize)]
struct HelloMessage{
    Hello : String,
}

fn HelloWorld(req : &Request,_ : Data) -> Outcome<'static>{
    let message = &HelloMessage{Hello:"Hello, Wordl!".to_string()};
    let jsonData = serde_json::to_string(&message).unwrap();
    Outcome::from(req,jsonData)
}


fn myRocket() -> rocket::Rocket{

    let helloWorldRoute = Route::ranked(1, Get, "/", HelloWorld);

    rocket::ignite()
        .mount("/",vec![helloWorldRoute])
}

fn main() {
    myRocket().launch();
}
