extern crate mysql;

use std::error::Error;

//struct in database
#[derive(Debug, PartialEq, Eq)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Item {
    item_name: String,
    quantity: Option<u32>,
    description: Option<String>,
    x_coord: Option<u8>,
    y_coord: Option<u8>,
}

impl Item {
	//all possible field names
	pub fn field_names() -> &'static str {
		"item_name,quantity,description,x_coord,y_coord"
	}
	
	//only return field name that are not NONE
	pub fn has_field_names(&self) -> &str {
		let mut field_names = String::from("item_name");
		match self.quantity {
			Some(_) => field_names.push_str(",quantity"),
		};
		match self.description {
			Some(_) => field_names.push_str(",description"),
		};
		match self.x_coord {
			Some(_) => field_names.push_str(",x_coord"),
		};
		match self.y_coord {
			Some(_) => field_names.push_str(",y_coord"),
		};
		
		return field_names.as_str()
	}
	
	//handle NONEs by not including in string at all
	pub fn fields(&self) -> &str {
		let mut field_values = String::from(self.item_name+",");
		match self.quantity {
			Some(q) => field_values.push_str(format!(",{}",q).as_str()),
		};
		match self.description {
			Some(d) => field_values.push_str(format!(",{}",d).as_str()),
		};
		match self.x_coord {
			Some(x) => field_values.push_str(format!(",{}",x).as_str()),
		};
		match self.y_coord {
			Some(y) => field_values.push_str(format!(",{}",y).as_str()),
		};
		
		return field_values.as_str()
	}
	
	pub fn fields_with_names(&self) -> &str {
		let mut field_names_values = String::from(format!("item_name='{}',",self.item_name).as_str());
		match self.quantity {
			Some(q) => field_names_values.push_str(format!(",quantity='{}'",q).as_str()),
		};
		match self.description {
			Some(d) => field_names_values.push_str(format!(",description='{}'",d).as_str()),
		};
		match self.x_coord {
			Some(x) => field_names_values.push_str(format!(",x_coord='{}'",x).as_str()),
		};
		match self.y_coord {
			Some(y) => field_names_values.push_str(format!(",y_coord='{}'",y).as_str()),
		};
		
		return field_names_values.as_str()
	}
	
	//check all option fields to ensure they are some (excluding description)
	pub fn valid_sql_insert(&self) -> Option<&str> {
		match self.quantity {
			None => return Some("quantity field not found"),
		};
		match self.x_coord {
			None => return Some("x_coord field not found"),
		};
		match self.y_coord {
			None => return Some("y_coord field not found"),
		};
		
		None
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