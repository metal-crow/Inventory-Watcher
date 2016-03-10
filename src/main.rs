extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct Greeting {
    msg: String
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
	println!("GET");
    let greeting = Greeting { msg: "Hello World".to_string() };
    let payload = json::encode(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

// Receive a message by POST and play it back.
fn set_greeting(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: Greeting = json::decode(&payload).unwrap();
    //let greeting = Greeting { msg: request.msg };
    //let payload = json::encode(&greeting).unwrap();
    Ok(Response::with((status::Ok)))
}

fn main() {
	let mut router = Router::new();
    router.get("/", hello_world);
    router.post("/set", set_greeting);
    
    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
