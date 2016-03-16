extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate mysql;
extern crate hyper;

mod dbmanager;

use dbmanager::{DatabaseManager, Item};
use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use rustc_serialize::json;
use std::sync::Arc;
use hyper::Client;

//TODO the two item get methods are the same except for the query. Use an enum, pass correct string when created by router?

//json request format for an item name only
#[derive(RustcEncodable, RustcDecodable)]
struct ItemRequest {
    item_name: String,
}
fn get_item_info(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	//get request json and convert to struct
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: ItemRequest = match json::decode(&payload) {
    	Ok(r) => r,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };
    
    let selected_items = match 
    database_manager.results_from_database(
    	format!("SELECT * from inventory WHERE item_name='{0}'", request.item_name)
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
    	format!("SELECT * from inventory WHERE item_name LIKE '%{0}%' OR description LIKE '%{0}%'", item_request.item_name_or_description)
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

	match database_manager.alter_database(format!("INSERT into inventory ({0}) VALUES ({1})",Item::field_names(), fields)) {
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

	match database_manager.alter_database(format!("UPDATE inventory SET {} WHERE item_name='{}'", item.fields_with_names(), item.get_item_name())) {
		None => Ok(Response::with(status::Ok)),
		Some(err) => Ok(Response::with((status::BadRequest, err.to_string())))
	}
}

//query coords from sql (secure) and send to the laser pointer
fn find_item_physical(request: &mut Request, database_manager : &DatabaseManager, laser_control : &Client) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: ItemRequest = match json::decode(&payload) {
    	Ok(r) => r,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };

	let selected_item: String = match 
    database_manager.results_from_database(
    	format!("SELECT * from inventory WHERE item_name='{0}'", request.item_name)
    ) 
    {
    	Ok(s_i) => match s_i.len() {
    		0 =>  return Ok(Response::with((status::BadRequest, "Item not found"))),
    		_ =>  String::from(format!("{0},{1}",s_i[0].get_item_xcoords(),s_i[0].get_item_ycoords())),
    	},
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string()))),
    };
    
    laser_control.post("localhost:3000/SetLaser").body(selected_item.as_str()).send().unwrap();    //safe to unwrap b/c we shouldnt get a response
    
    Ok(Response::with((status::Ok)))
}

fn main() {
	let opts = match dbmanager::get_opts() {
		Err(err) => panic!("Error reading MySQL settings file: {:?}",err),
		Ok(o) => o,
	};
	println!("{:?}",opts);
	let database_manager_info = Arc::new(
		DatabaseManager {
			pool: mysql::Pool::new(opts).unwrap(),
		});
	
	let database_manager_search = database_manager_info.clone();
	let database_manager_add = database_manager_info.clone();
	let database_manager_update = database_manager_info.clone();
	let database_manager_find = database_manager_info.clone();

    let laser_control = Client::new();
    
	let mut router = Router::new();
	
    router.post("/ItemInfo", move |r: &mut Request| get_item_info(r, &database_manager_info));
    router.post("/ItemSearch" , move |r: &mut Request| search_for_item(r, &database_manager_search));
    router.post("/ItemAdd" , move |r: &mut Request| add_item_to_inventory(r, &database_manager_add));
    router.post("/ItemUpdate" , move |r: &mut Request| update_item_in_inventory(r, &database_manager_update));
    router.post("/ItemFind" , move |r: &mut Request| find_item_physical(r, &database_manager_find, &laser_control));

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
