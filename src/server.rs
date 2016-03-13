extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate mysql;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use rustc_serialize::json;
use std::error::Error;
use std::sync::Arc;

//struct in database
#[derive(Debug, PartialEq, Eq)]
#[derive(RustcEncodable, RustcDecodable)]
struct Item {
    item_name: String,
    quantity: i32,
    description: Option<String>,
}

impl Item {
	fn new(item_name : String, quantity: i32, description: Option<String>) -> Item {
		Item{ item_name: item_name, quantity: quantity, description: description }
	}
}

static MYSQL_DB_COLS: &'static str = "item_name,quantity,description";

struct DatabaseManager{
	pool : mysql::Pool,
}

impl DatabaseManager {
	//handles querying the database, and returing an array of the results in Item form. Only function that has access to Pool
	fn results_from_database(&self, query : String) -> Result<Vec<Item>, mysql::Error> {
		let mut statement = match self.pool.prepare(query) {
			Ok(s) => s,
			Err(err) => return Err(err),
		};
		
		let selected_items: Result<Vec<Item>, mysql::Error> = statement.execute(()).map(|result| {
	    	// In this closure we sill map `QueryResult` to `Vec`
	        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
	        // will map each `MyResult` to contained `row` (no proper error handling)
	        // and second call to `map` will map each `row` to `Payment`
	        result.map(|x| x.unwrap()).map(|row| {
	        	//TODO there should be a better way to do this	
	            let (item_name, quantity, description) = mysql::from_row(row);
	            Item {
	                item_name: item_name,
	                quantity: quantity,
	                description: description,
	            }
	        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec`
	    });
	    
	    return selected_items
	}
	
	//handle inserting into the database or any other action that doesnt return a result, but can error
	fn insert_into_database(&self, items: Vec<Item>) -> Vec<String> {
		//allow execute of valid comands, but report errors
		let mut errors: Vec<String> = Vec::new();

		//insert into database
		for mut stmt in self.pool.prepare(format!("INSERT into test.inventory ({0}) VALUES (?,?,?)",MYSQL_DB_COLS)).into_iter() {
	        for i in items.iter() {
	            match stmt.execute((&i.item_name, i.quantity, &i.description)) {
	            	Ok(_) => continue,
	            	Err(err) => errors.push(err.to_string()),
	            }
	        }
	    }
		
		return errors;
	}

}




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
