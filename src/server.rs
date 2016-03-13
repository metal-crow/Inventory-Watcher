extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate mysql;

mod dbmanager;

use dbmanager::{DatabaseManager, Item};
use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use rustc_serialize::json;
use std::sync::Arc;

//json request format for get_item
#[derive(RustcEncodable, RustcDecodable)]
struct GetItemRequest {
    item_name: String,
}
fn get_item_info(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	//get request json and convert to struct
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: GetItemRequest = match json::decode(&payload) {
    	Ok(r) => r,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };
    
    let selected_items = match 
    database_manager.results_from_database(
    	format!("SELECT * from test.inventory WHERE item_name=\"{0}\"", request.item_name)
    ) 
    {
    	Ok(s_i) => s_i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };
    
    //convert the sql result to json and return
    let payload = json::encode(&selected_items).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn add_items_to_inventory(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	//takes json array of Items
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let items: Vec<Item> = match json::decode(&payload) {
    	Ok(i) => i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };

	let errors = database_manager.insert_into_database(items);

	//take any errors, convert to json, and return
    let payload = json::encode(&errors).unwrap();

    Ok(Response::with((status::Ok, payload)))
}

//json request format for search_for_item
#[derive(RustcEncodable, RustcDecodable)]
struct SearchItemRequest {
    item_name: String,//find any item name that contains this
    description: Option<String>,//or any item description that contains this
}
fn search_for_item(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let item_request: SearchItemRequest = match json::decode(&payload) {
    	Ok(i) => i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };
    
    let selected_items = match 
    database_manager.results_from_database(
    	format!("SELECT * from test.inventory WHERE item_name LIKE \"%{0}%\"", item_request.item_name)
    ) 
    {
    	Ok(s_i) => s_i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };
    
    let payload = json::encode(&selected_items).unwrap();
    Ok(Response::with((status::Ok,payload)))
}

fn main() {
	let database_manager_info = Arc::new(DatabaseManager {
		pool: mysql::Pool::new("mysql://root:test@localhost:3306").unwrap(),
	});
	let database_manager_add = database_manager_info.clone();
	let database_manager_search = database_manager_info.clone();

	let mut router = Router::new();
	
    router.post("/ItemInfo", move |r: &mut Request| get_item_info(r, &database_manager_info));
    router.post("/ItemAdd" , move |r: &mut Request| add_items_to_inventory(r, &database_manager_add));
    router.post("/ItemSearch" , move |r: &mut Request| search_for_item(r, &database_manager_search));

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
