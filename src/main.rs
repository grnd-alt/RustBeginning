#![feature(proc_macro_hygiene, decl_macro)]

use std::collections::HashMap;

use rocket::{Catcher, Request, response::{self, status::Custom, Responder, Redirect}, http::{Status, Cookies, Cookie}};

#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct Message{
    message: String,
}

#[get("/")]
fn index(cookies: Cookies) -> Template {
    let cookie = cookies.get("message");
    let mut context = 
    HashMap::new();
    if let Some(ref cookie) = cookie {
        context.insert("message", cookie.value());
    }
}

#[get("/CookieGet")]
fn get_cookie(cookies: Cookies) -> Option<String>{
    cookies.get("message")
        .map(|cookie| format!("User ID: {}", cookie.value()))
}

#[get("/TOM")]
fn tom() -> &'static str {
    "<div><h1>HALLO BRO</h1></div>"
}

#[post("/setCookie",data="<message>")]
fn set_cookie(mut cookies: Cookie,message: Form<Message>) -> Redirect{
    cookies.add(Cookie::new("message", message.into_inner().message))
}

#[get("/rob")]
fn robert() -> &'static str { "ROBERT FICK DICH mehr"}

fn not_found_handler<'r>(req: &'r Request) -> response::Result<'r> {
    let res = Custom(Status::NotFound, format!("Couldn't find: {}", req.uri()));
    res.respond_to(req)
}

fn rocket() -> rocket::Rocket {
    let not_found_catcher = Catcher::new(404, not_found_handler);
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/",routes![robert])
    .mount("/",routes![tom])
    .mount("/",routes![get_cookie])
    .register(vec![not_found_catcher])
}


fn main() {
    rocket().launch();
}
