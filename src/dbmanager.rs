extern crate mysql;
extern crate ini;

use mysql::conn::Opts;

#[derive(Debug, PartialEq, Eq)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Item {
	pub item_key: Option<u64>,
    pub item_name: String,
    pub quantity: u32,
    pub description: String,
    x_coord: u32,
    y_coord: u32,
    width: u32,
    height: u32,
}

impl Item {
	//all possible field names
	pub fn field_names() -> &'static str {
		"item_name,quantity,description,x_coord,y_coord,width,height"
	}
	
	//returns comma seperated string of the values of the struct's fields
	pub fn fields(&self) -> String {
		return format!("
		'{0}',{1},'{2}',{3},{4},{5},{6}",
		self.item_name.as_str(),
		self.quantity,
		self.description.as_str(),
		self.x_coord,
		self.y_coord,
		self.width,
		self.height);
	}
	
	//returns string in format VARNAME=VARVALUE,...
	pub fn fields_with_names(&self) -> String {
		return format!("
		item_name='{0}',quantity={1},description='{2}',x_coord={3},y_coord={4},width={5},height={6}",
		self.item_name.as_str(),
		self.quantity,
		self.description.as_str(),
		self.x_coord,
		self.y_coord,
		self.width,
		self.height);
	}
	
	pub fn get_item_key(&self) -> u64 {
		self.item_key.unwrap()
	}
}

pub struct DatabaseManager{
	pub pool : mysql::Pool,
}

impl DatabaseManager {
	//handles querying the database, and returing an array of the results in Item form.
	//since a passed ref has to have a lifetime at a higher level, the results vec must be supplied
	pub fn results_from_database<'a>(&self, query : String, results: & 'a mut Vec<Item>) -> Option<mysql::Error> {
		let mut statement = match self.pool.prepare(query) {
			Ok(s) => s,
			Err(err) => return Some(err),
		};
		
		let row_iterator = match statement.execute(()) {
			Ok(r) => r,
			Err(e) => return Some(e)
		};
		
		for row in row_iterator {
			let row_res = match row {
				Ok(r) => r,
				Err(e) =>  return Some(e),
			};
			let (item_key,item_name, quantity, description, x_coord, y_coord, width, height) = mysql::from_row(row_res);
            //push the mapped item onto the passed vec reference
            results.push(
            	Item {
	            	item_key: item_key,
	                item_name: item_name,
	                quantity: quantity,
	                description: description,
	                x_coord: x_coord,
	                y_coord: y_coord,
	                width: width,
	                height: height,
            	}
            );
		}
		
		None
	}
	
	//handle inserting into the database or any other action that doesnt return a result, but can error
	pub fn alter_database(&self, query : String) -> Option<mysql::Error> {
		match self.pool.prep_exec(query, ()) {
			Ok(_) => None,
			Err(err) => return Some(err),
		}
	}
}

#[derive(Debug)]
pub struct EmailSettings {
	pub restocker_email: String,
	pub mail_server: String,
	pub mail_server_port: u16,
	pub mail_username: String,
	pub mail_password: String,
}
#[derive(Debug)]
pub struct Settings {
	pub opts: Opts,
	pub dns: String,
	pub email_settings: EmailSettings,
}

//read settings from settings.ini
pub fn get_opts() -> Result<Settings,String> {
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
	/*let inventory_table_name = match mysql_settings.get("inventory_table_name") {
		Some(s) => s,
		None => return Err("inventory_table_name variable not found".to_string()),
	};
	let restocking_table_name = match mysql_settings.get("restocking_table_name") {
		Some(s) => s,
		None => return Err("restocking_table_name variable not found".to_string()),
	};*/
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
		
	let email_settings = match conf.section(Some("Email".to_owned())) {
		Some(s) => s,
		None => return Err("Email section not found".to_string()),
	};

	let restocker_email = match email_settings.get("restocker_email") {
		Some(s) => s,
		None => return Err("restocker_email variable not found".to_string()),
	};
	
	let mail_server = match email_settings.get("mail_server") {
		Some(s) => s,
		None => return Err("mail_server variable not found".to_string()),
	};
	
	let mail_server_port = match email_settings.get("mail_server_port") {
		Some(s) => match s.parse::<u16>() {
			Ok(s) => s,
			Err(_) => return Err("mail_server_port variable not a valid number".to_string()),
		},
		None => return Err("mail_server_port variable not found".to_string()),
	};
			
	let mail_username = match email_settings.get("mail_username") {
		Some(s) => s,
		None => return Err("mail_username variable not found".to_string()),
	};
	
	let mail_password = match email_settings.get("mail_password") {
		Some(s) => s,
		None => return Err("mail_password variable not found".to_string()),
	};
			
	let server_settings = match conf.section(Some("Server".to_owned())) {
		Some(s) => s,
		None => return Err("Server section not found".to_string()),
	};

	let dns_name = match server_settings.get("dns_name") {
		Some(s) => s,
		None => return Err("dns_name variable not found".to_string()),
	};

	Ok(
		Settings{
			opts: Opts {
			    user: Some(user.to_string()),
			    pass: Some(pass.to_string()),
			    db_name: Some(db_name.to_string()),
			    ip_or_hostname: Some(ip_or_hostname.to_string()),
			    tcp_port: port,
			    ..Default::default()
			},
			dns: dns_name.to_string(),
			email_settings: EmailSettings {
				restocker_email: restocker_email.to_string(),
				mail_server: mail_server.to_string(),
				mail_server_port: mail_server_port,
				mail_username: mail_username.to_string(),
				mail_password: mail_password.to_string(),
			}
		}
	)
}