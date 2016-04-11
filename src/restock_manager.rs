extern crate mysql;

use dbmanager::{DatabaseManager, EmailSettings};
use std::thread;
use std::sync::{Arc};

pub struct RestockingManager {
	restocking_database: DatabaseManager,
	email_settings: EmailSettings
}

impl RestockingManager {
	
	pub fn new_restock_manager(restocking_database: DatabaseManager, email_settings: EmailSettings) -> Result<Arc<RestockingManager>,String> {	    
	    let restocking_manager = Arc::new(RestockingManager {
			restocking_database: restocking_database,
			email_settings: email_settings,
		});
	    let restocking_manager_return = restocking_manager.clone();
	    
	    //spawn thread for starting email
	    thread::spawn(move || {restocking_manager.send_restocking_email()});
	    
	    Ok(restocking_manager_return)
	}
	
	//mark item for restocking if not marked already(checking handled by the sql table)
	pub fn add_item_for_restocking(&self, item_key: u64) -> Option<String> {
		match self.restocking_database.alter_database(format!("INSERT into restocking (item_key) VALUES ({})", item_key)) {
			None => return None,
			Some(err) => match err {
				mysql::error::Error::MySqlError(ref err) => match err.code {
					/*AUUGH FUCK CANT GET THIS TO CAST mysql::ServerError::ER_DUP_ENTRY*/ 1062u16 => return None,//ignore duplicate entry error, because its effectily same as success(entry is already in)
					_ => return Some(err.to_string()),
				},
				_ => return Some(err.to_string()),
			}
		}
	}
	
	//run inside a thread. Checks if its time to send the email, then sleeps the thread till it needs to send.
	//sending the email removes all items from the restocking list, and inserts them into the email
	pub fn send_restocking_email(&self){
		println!("hi");
		/*
		let mut selected_item = Vec::new();
	    match database_manager.results_from_database(
	    	format!("SELECT * from inventory WHERE item_key={0}", item.item_key),
	    	&mut selected_item
	    ) 
	    {
	    	None => match selected_item.len() {
	    		1 => {},
	    		_ => return Ok(Response::with((status::InternalServerError, "Too many results found (more that 1 item has the same primay key. Your MySQL schema is incorrect)".to_string())))
	    	},
	    	Some(err) => return Ok(Response::with((status::BadRequest, err.to_string())))
	    };
	
		let email = EmailBuilder::new()
	                    .to(email_settings.restocker_email.as_str())
	                    .from("no-reply@InventoryManager")
	                    .body(format!("There are currently {} {} left in stock, and a user requested we get more.\nDescription: {}",selected_item[0].quantity, selected_item[0].item_name, selected_item[0].description).as_str())
	                    .subject(format!("{} needs restocking",selected_item[0].item_name).as_str())
	                    .build()
	                    .unwrap();
	    //TODO can i build this on startup and pass ref to method?
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
	
		let result = mailer.send(email);
		mailer.close();// Explicitely close the SMTP transaction as we enabled connection reuse*/
		
	}
}