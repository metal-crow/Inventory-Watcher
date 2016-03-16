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

//TODO the two item get methods are the same except for the query. Use an enum, pass correct string when created by router?

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
    	format!("SELECT * from test.inventory WHERE item_name='{0}'", request.item_name)
    ) 
    {
    	Ok(s_i) => s_i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };
    
    //convert the sql result to json and return
    let payload = json::encode(&selected_items).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

//json request format for search_for_item
#[derive(RustcEncodable, RustcDecodable)]
struct SearchItemRequest {
    item_name_or_description: String,//find any item name OR description that contains this
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
    	format!("SELECT * from test.inventory WHERE item_name LIKE '%{0}%' OR description LIKE '%{0}%'", item_request.item_name_or_description)
    ) 
    {
    	Ok(s_i) => s_i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };
    
    let payload = json::encode(&selected_items).unwrap();
    Ok(Response::with((status::Ok,payload)))
}

//dont allow any NONE values 
fn add_item_to_inventory(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let item: Item = match json::decode(&payload) {
    	Ok(i) => i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };
	
	let fields = match item.fields() {
		Ok(r) => r,
		Err(err) => return Ok(Response::with((status::BadRequest, err.to_string()))),
	};

	match database_manager.alter_database(format!("INSERT into test.inventory ({0}) VALUES ({1})",Item::field_names(), fields)) {
		None => Ok(Response::with(status::Ok)),
		Some(err) => Ok(Response::with((status::BadRequest, err.to_string())))
	}
}

//allows anything except item_name to be NONE(NONE=do not update)
fn update_item_in_inventory(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let item: Item = match json::decode(&payload) {
    	Ok(i) => i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };

	match database_manager.alter_database(format!("UPDATE test.inventory SET {} WHERE item_name='{}'", item.fields_with_names(), item.get_item_name())) {
		None => Ok(Response::with(status::Ok)),
		Some(err) => Ok(Response::with((status::BadRequest, err.to_string())))
	}
}

fn main() {
	let database_manager_info = Arc::new(DatabaseManager {
		pool: mysql::Pool::new("mysql://root:test@localhost:3306").unwrap(),
	});
	let database_manager_search = database_manager_info.clone();
	let database_manager_add = database_manager_info.clone();
	let database_manager_update = database_manager_info.clone();

	let mut router = Router::new();
	
    router.post("/ItemInfo", move |r: &mut Request| get_item_info(r, &database_manager_info));
    router.post("/ItemSearch" , move |r: &mut Request| search_for_item(r, &database_manager_search));
    router.post("/ItemAdd" , move |r: &mut Request| add_item_to_inventory(r, &database_manager_add));
    router.post("/ItemUpdate" , move |r: &mut Request| update_item_in_inventory(r, &database_manager_update));

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
