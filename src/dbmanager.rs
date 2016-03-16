extern crate mysql;

use std::error::Error;
use mysql::conn::Opts;
use std::io::prelude::*;
use std::fs::File;

//struct in database
#[derive(Debug, PartialEq, Eq)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Item {
    item_name: String,
    quantity: Option<u32>,
    description: Option<String>,
    x_coord: Option<u32>,
    y_coord: Option<u32>,
}

impl Item {
	//all possible field names
	pub fn field_names() -> &'static str {
		"item_name,quantity,description,x_coord,y_coord"
	}
	
	//returns comma seperated string of the values of the struct's fields
	//handle NONEs by not including in string
	//also return what fields were none(if any), in error
	pub fn fields(&self) -> Result<String,String> {
		let mut field_values = String::from(format!("'{}'",self.item_name.as_str()));
		let mut errors = String::new();
		
		match self.quantity {
			None => errors.push_str("quantity field not found,"),
			Some(q) => field_values.push_str(format!(",{}",q).as_str()),
		};
		match self.description.as_ref() {
			None => errors.push_str("description field not found,"),
			Some(d) => field_values.push_str(format!(",'{}'",d).as_str()),
		};
		match self.x_coord {
			None => errors.push_str("x_coord field not found,"),
			Some(x) => field_values.push_str(format!(",{}",x).as_str()),
		};
		match self.y_coord {
			None => errors.push_str("y_coord field not found,"),
			Some(y) => field_values.push_str(format!(",{}",y).as_str()),
		};
		
		match errors.as_ref() {
			"" => return Ok(field_values),
			_ => return Err(errors)
		}
	}
	
	//returns string in format VARNAME=VARVALUE,...
	//handle NONEs by not including in string
	pub fn fields_with_names(&self) -> String {
		let mut field_names_values = String::from(format!("item_name='{}'",self.item_name).as_str());
		match self.quantity {
			None => {},
			Some(q) => field_names_values.push_str(format!(",quantity={}",q).as_str()),
		};
		match self.description.as_ref() {
			None => {},
			Some(d) => field_names_values.push_str(format!(",description='{}'",d).as_str()),
		};
		match self.x_coord {
			None => {},
			Some(x) => field_names_values.push_str(format!(",x_coord={}",x).as_str()),
		};
		match self.y_coord {
			None => {},
			Some(y) => field_names_values.push_str(format!(",y_coord={}",y).as_str()),
		};
		
		return field_names_values
	}
	
	pub fn get_item_name(&self) -> &str {
		self.item_name.as_str()
	}
	
	pub fn get_item_xcoords(&self) -> u32 {
		self.x_coord.unwrap()
	}
	
	pub fn get_item_ycoords(&self) -> u32 {
		self.y_coord.unwrap()
	}

}

pub struct DatabaseManager{
	pub pool : mysql::Pool,
}

impl DatabaseManager {
	//handles querying the database, and returing an array of the results in Item form.
	pub fn results_from_database(&self, query : String) -> Result<Vec<Item>, mysql::Error> {
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
	            let (item_name, quantity, description, x_coord, y_coord) = mysql::from_row(row);
	            Item {
	                item_name: item_name,
	                quantity: quantity,
	                description: description,
	                x_coord: x_coord,
	                y_coord: y_coord,
	            }
	        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec`
	    });
	    
	    return selected_items
	}
	
	//handle inserting into the database or any other action that doesnt return a result, but can error
	pub fn alter_database(&self, query : String) -> Option<mysql::Error> {
		match self.pool.prep_exec(query, ()) {
			Ok(_) => None,
			Err(err) => return Some(err),
		}
	}
}

//read mysql settings from settings.txt
pub fn get_opts () -> Opts {
	let mut settings = File::open("settings.txt").unwrap();
	let mut passwd = String::new();
	settings.read_to_string(&mut passwd);
	
	Opts {
	    user: Some("root".to_string()),
	    pass: Some(passwd),
	    db_name: Some("test".to_string()),
	    ip_or_hostname: Some("localhost".to_string()),
	    tcp_port: 3306,
	    ..Default::default()
	}
}