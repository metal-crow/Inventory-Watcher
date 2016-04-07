extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate mysql;
extern crate staticfile;
extern crate mount;
extern crate lettre;

mod dbmanager;

use dbmanager::{DatabaseManager, Item, EmailSettings};
use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use rustc_serialize::json;
use std::sync::Arc;
use std::path::Path;
use staticfile::Static;
use mount::Mount;
use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::EmailTransport;

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
struct ItemRequest {
    item_key: u64,
}
fn delete_item_in_inventory(request: &mut Request, database_manager : &DatabaseManager) -> IronResult<Response> {
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let item: ItemRequest = match json::decode(&payload) {
    	Ok(i) => i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };

	match database_manager.alter_database(format!("DELETE from inventory WHERE item_key='{}'", item.item_key)) {
		None => Ok(Response::with(status::Ok)),
		Some(err) => Ok(Response::with((status::BadRequest, err.to_string())))
	}
}

fn alert_item_restock(request: &mut Request, database_manager : &DatabaseManager, email_settings : &EmailSettings) -> IronResult<Response> {
	println!("restock");
	let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let item: ItemRequest = match json::decode(&payload) {
    	Ok(i) => i,
    	Err(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
    };

	let email = EmailBuilder::new()
                    .to(email_settings.restocker_email.as_str())
                    .from("no-reply@InventoryManager")
                    .body("Hello World!")
                    .subject("Needs Restocking!")
                    .build()
                    .unwrap();
	// Connect to a remote server on a custom port
	let mut mailer = SmtpTransportBuilder::new((email_settings.mail_server.as_str(),email_settings.mail_server_port)).unwrap()
    // Set the name sent during EHLO/HELO, default is `localhost`
    .hello_name("my.hostname.tld")
    // Add credentials for authentication
    .credentials(email_settings.mail_username.as_str(), email_settings.mail_password.as_str())
    // Specify a TLS security level. You can also specify an SslContext with
    // .ssl_context(SslContext::Ssl23)
    .security_level(SecurityLevel::AlwaysEncrypt)
    // Enable SMTPUTF8 is the server supports it
    .smtp_utf8(true)
    // Configure accepted authetication mechanisms
    .authentication_mechanisms(vec![Mechanism::CramMd5])
    // Enable connection reuse
    .connection_reuse(true).build();
	println!("connected");

	let result_1 = mailer.send(email);
	println!("{:?}",result_1.is_ok());
	
	mailer.close();
	
	Ok(Response::with((status::Ok)))
}

fn main() {
	let settings = match dbmanager::get_opts() {
		Err(err) => panic!("Error reading settings file: {:?}",err),
		Ok(o) => o,
	};
	println!("{:?}",settings);
	
	let settings_opts = settings.opts;
	let settings_email = settings.email_settings;
	let settings_dns = settings.dns;
	
	let database_manager_info = Arc::new(
		DatabaseManager {
			pool: match mysql::Pool::new(settings_opts) {
				Ok(p) => p,
				Err(_) => panic!("Could not connect to MySQL database (Is the server up? Is your username/password correct?)"),
			},
		});
	let database_manager_search = database_manager_info.clone();
	let database_manager_add = database_manager_info.clone();
	let database_manager_update = database_manager_info.clone();
	let database_manager_delete = database_manager_info.clone();
	let database_manager_alert = database_manager_info.clone();

	let mut mount = Mount::new();
	let mut router = Router::new();
	
	//REST API endpoints
    router.post("/ItemSearch" , move |r: &mut Request| search_for_item(r, &database_manager_search));
    router.post("/ItemAdd" , move |r: &mut Request| add_item_to_inventory(r, &database_manager_add));
    router.post("/ItemUpdate" , move |r: &mut Request| update_item_in_inventory(r, &database_manager_update));
    router.post("/ItemDelete" , move |r: &mut Request| delete_item_in_inventory(r, &database_manager_delete));
    router.post("/ItemAlert" , move |r: &mut Request| alert_item_restock(r, &database_manager_alert, &settings_email));

	//these endpoints serves all the static html/js client view stuff. Then the js queries api endpoints
 	mount.mount("/index", Static::new(Path::new("public/index.html")));
 	mount.mount("/public", Static::new(Path::new("public")));
	mount.mount("/", router);
    
    Iron::new(mount).http(format!("{}:80",settings_dns).as_str()).unwrap();
}