extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate mysql;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use rustc_serialize::json;
use mysql as my;

//struct in database
#[derive(Debug, PartialEq, Eq)]
#[derive(RustcEncodable, RustcDecodable)]
struct Item {
    item_name: String,
    quantity: i32,
    description: Option<String>,
}

//json request format for get_item
#[derive(RustcEncodable, RustcDecodable)]
struct GetItemRequest {
    item_name: String,
}
fn get_item_info(request: &mut Request, pool: &my::Pool) -> IronResult<Response> {
	//get request json and convert to struct
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: GetItemRequest = json::decode(&payload).unwrap();
    
    let mut statement = pool.prepare("SELECT item_name,quantity,description from test.inventory WHERE item_name=?").unwrap();

    //execute the sql query with the data from the struct
    let selected_items: Vec<Item> = statement.execute((request.item_name,))    
    .map(|result| {
    	// In this closure we sill map `QueryResult` to `Vec<Payment>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Payment`
        result.map(|x| x.unwrap()).map(|row| {
            let (item_name, quantity, description) = my::from_row(row);
            Item {
                item_name: item_name,
                quantity: quantity,
                description: description,
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`
    
    //convert the sql result to json and return
    let payload = json::encode(&selected_items).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

// Receive a message by POST and play it back.
fn add_item_to_inventory(request: &mut Request, pool: &my::Pool) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

fn main() {
	let pool = my::Pool::new("mysql://root:test@localhost:3306").unwrap();
	let pool_clone = pool.clone();

	let mut router = Router::new();
	
    router.post("/ItemInfo", move |r: &mut Request| get_item_info(r, &pool));
    
    router.post("/ItemAdd" , move |r: &mut Request| add_item_to_inventory(r, &pool_clone));
        
    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
