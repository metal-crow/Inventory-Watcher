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

//TODO the two item get methods are the same except for the query. Use an enum, pass correct string when created by router?

//json request format for an item name only
#[derive(RustcEncodable, RustcDecodable)]
struct ItemRequest {
    item_key: String,
}
fn get_item_info(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	println!("get");
	//get request json and convert to struct
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: ItemRequest = match json::decode(&payload) {
    	Ok(r) => r,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };
    
    let selected_items = match 
    database_manager.results_from_database(
    	format!("SELECT * from inventory WHERE item_key='{0}'", request.item_key)
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
	println!("search");
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
	println!("add");
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
	println!("update");
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

//query coords for image from sql and return
fn find_item_physical(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	println!("find");
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: ItemRequest = match json::decode(&payload) {
    	Ok(r) => r,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };

	let item_coords: String = match 
    database_manager.results_from_database(
    	format!("SELECT * from inventory WHERE item_key='{0}'", request.item_key)
    ) 
    {
    	Ok(s_i) => match s_i.len() {
    		0 =>  return Ok(Response::with((status::BadRequest, "Item not found"))),
    		_ =>  String::from(format!("{0},{1}",s_i[0].get_item_xcoords(),s_i[0].get_item_ycoords())),
    	},
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string()))),
    };
        
    Ok(Response::with((status::Ok,item_coords)))
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
	let database_manager_find = database_manager_info.clone();

	let mut mount = Mount::new();
	let mut router = Router::new();
	//these endpoints serves all the static html/js client view stuff. Then the js queries api endpoints
	mount.mount("/", Static::new(Path::new("public/index.html")));
	mount.mount("/public", Static::new(Path::new("public")));
	
	//REST API endpoints
    router.post("/ItemInfo", move |r: &mut Request| get_item_info(r, &database_manager_info));
    router.post("/ItemSearch" , move |r: &mut Request| search_for_item(r, &database_manager_search));
    router.post("/ItemAdd" , move |r: &mut Request| add_item_to_inventory(r, &database_manager_add));
    router.post("/ItemUpdate" , move |r: &mut Request| update_item_in_inventory(r, &database_manager_update));
    router.post("/ItemFind" , move |r: &mut Request| find_item_physical(r, &database_manager_find));

	mount.mount("", router);
    Iron::new(mount).http(format!("{}:80",opts.1).as_str()).unwrap();
}
