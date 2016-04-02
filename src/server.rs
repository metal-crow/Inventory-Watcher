extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate mysql;
extern crate staticfile;
extern crate mount;

mod dbmanager;

use dbmanager::{DatabaseManager, Item};
use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use rustc_serialize::json;
use std::sync::Arc;
use std::path::Path;
use staticfile::Static;
use mount::Mount;

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

fn add_item_to_inventory(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let item: Item = match json::decode(&payload) {
    	Ok(i) => i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };

	match database_manager.alter_database(format!("INSERT into inventory ({0}) VALUES ({1})",Item::field_names(), item.fields())) {
		None => Ok(Response::with(status::Ok)),
		Some(err) => Ok(Response::with((status::BadRequest, err.to_string())))
	}
}

fn update_item_in_inventory(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let item: Item = match json::decode(&payload) {
    	Ok(i) => i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };

	match database_manager.alter_database(format!("UPDATE inventory SET {} WHERE item_key='{}'", item.fields_with_names(), item.get_item_key())) {
		None => Ok(Response::with(status::Ok)),
		Some(err) => Ok(Response::with((status::BadRequest, err.to_string())))
	}
}

#[derive(RustcEncodable, RustcDecodable)]
struct DeleteItemRequest {
    item_key: u64,
}
fn delete_item_in_inventory(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let item: DeleteItemRequest = match json::decode(&payload) {
    	Ok(i) => i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };

	match database_manager.alter_database(format!("DELETE from inventory WHERE item_key='{}'", item.item_key)) {
		None => Ok(Response::with(status::Ok)),
		Some(err) => Ok(Response::with((status::BadRequest, err.to_string())))
	}
}

fn main() {
	let opts = match dbmanager::get_opts() {
		Err(err) => panic!("Error reading settings file: {:?}",err),
		Ok(o) => o,
	};
	println!("{:?}",opts);
	
	let database_manager_info = Arc::new(
		DatabaseManager {
			pool: match mysql::Pool::new(opts.0) {
				Ok(p) => p,
				Err(_) => panic!("Could not connect to MySQL database (Is the server up? Is your username/password correct?)"),
			},
		});
	let database_manager_search = database_manager_info.clone();
	let database_manager_add = database_manager_info.clone();
	let database_manager_update = database_manager_info.clone();
	let database_manager_delete = database_manager_info.clone();

	let mut mount = Mount::new();
	let mut router = Router::new();
	
	//REST API endpoints
    router.post("/ItemSearch" , move |r: &mut Request| search_for_item(r, &database_manager_search));
    router.post("/ItemAdd" , move |r: &mut Request| add_item_to_inventory(r, &database_manager_add));
    router.post("/ItemUpdate" , move |r: &mut Request| update_item_in_inventory(r, &database_manager_update));
    router.post("/ItemDelete" , move |r: &mut Request| delete_item_in_inventory(r, &database_manager_delete));

	//these endpoints serves all the static html/js client view stuff. Then the js queries api endpoints
 	mount.mount("/index", Static::new(Path::new("public/index.html")));
 	mount.mount("/public", Static::new(Path::new("public")));
	mount.mount("/", router);
    
    Iron::new(mount).http(format!("{}:80",opts.1).as_str()).unwrap();
}