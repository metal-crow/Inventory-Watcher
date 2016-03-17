extern crate mysql;
extern crate ini;

use mysql::conn::Opts;

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

//read mysql and laser control settings from settings.ini
pub fn get_opts() -> Result<(Opts,String),String> {
	let conf = match ini::Ini::load_from_file("settings.ini") {
		Ok(f) => f,
		Err(_) => return Err("settings.ini file not found".to_string()),
	};
	let mysql_settings = match conf.section(Some("MySQL".to_owned())) {
		Some(s) => s,
		None => return Err("MySQL section not found".to_string()),
	};
	let user = match mysql_settings.get("user") {
		Some(s) => s,
		None => return Err("user variable not found".to_string()),
	};
	let pass = match mysql_settings.get("password") {
		Some(s) => s,
		None => return Err("password variable not found".to_string()),
	};
	let db_name = match mysql_settings.get("database_name") {
		Some(s) => s,
		None => return Err("database_name variable not found".to_string()),
	};
	let ip_or_hostname = match mysql_settings.get("ip_or_hostname") {
		Some(s) => s,
		None => return Err("ip_or_hostname variable not found".to_string()),
	};
	let port = match mysql_settings.get("port") {
		Some(s) => match s.parse::<u16>() {
			Ok(s) => s,
			Err(_) => return Err("port variable not a valid number".to_string()),
		},
		None => return Err("port variable not found".to_string()),
	};
	let raspi_settings = match conf.section(Some("RasPi".to_owned())) {
		Some(s) => s,
		None => return Err("RasPi section not found".to_string()),
	};
	let raspi_ip_or_host = match raspi_settings.get("rasPi_ip_or_host") {
		Some(s) => s,
		None => return Err("rasPi_ip_or_host variable not found".to_string()),
	};

	Ok(
		(
			Opts {
			    user: Some(user.to_string()),
			    pass: Some(pass.to_string()),
			    db_name: Some(db_name.to_string()),
			    ip_or_hostname: Some(ip_or_hostname.to_string()),
			    tcp_port: port,
			    ..Default::default()
			},
		raspi_ip_or_host.to_string()
		)
	)
}