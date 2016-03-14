extern crate mysql;

use std::error::Error;

//struct in database
#[derive(Debug, PartialEq, Eq)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Item {
    item_name: String,
    quantity: u32,
    description: Option<String>,
}

impl Item {
	pub fn field_names() -> &'static str {
		"item_name,quantity,description"
	}
	
	pub fn fields(&self) -> String {
		format!("{0},{1},{2}",self.item_name, self.quantity, self.description.clone().unwrap_or("".to_string()))
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
	pub fn alter_database(&self, query : String) -> Option<mysql::Error> {
		match self.pool.prep_exec(query, ()) {
			Ok(_) => None,
			Err(err) => return Some(err),
		}
	}
}