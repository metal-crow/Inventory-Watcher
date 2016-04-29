extern crate mysql;
extern crate time;
extern crate lettre;

use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel,SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::EmailTransport;
use dbmanager::{DatabaseManager, EmailSettings, Item};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
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
		// Connect to a remote server on a custom port
		let mut mailer = SmtpTransportBuilder::new((self.email_settings.mail_server.as_str(),self.email_settings.mail_server_port)).unwrap()
	    // Set the name sent during EHLO/HELO, default is `localhost`
	    .hello_name("my.hostname.tld")
	    // Add credentials for authentication
	    .credentials(self.email_settings.mail_username.as_str(), self.email_settings.mail_password.as_str())
	    // Specify a TLS security level. You can also specify an SslContext with
	    // .ssl_context(SslContext::Ssl23)
	    .security_level(SecurityLevel::AlwaysEncrypt)
	    // Enable SMTPUTF8 is the server supports it
	    .smtp_utf8(true)
	    // Configure accepted authetication mechanisms
	    .authentication_mechanisms(vec![Mechanism::CramMd5])
	    // Enable connection reuse
	    .connection_reuse(true).build();
		    
		loop {
			//get the time from now till next friday, 5pm
			let mut alert_restock_time = time::now();
			alert_restock_time = match alert_restock_time.tm_wday<=5 && alert_restock_time.tm_hour<17 {
				true => alert_restock_time + self::time::Duration::days(5 - alert_restock_time.tm_wday as i64),
				false => alert_restock_time + self::time::Duration::days(6),//saturday, 6 days to friday
			};
			alert_restock_time.tm_hour = 17;//5pm
			alert_restock_time.tm_min = 0;//start of 5pm
			alert_restock_time.tm_sec = 0;
			
			let time_to_wait = Duration::new(((alert_restock_time - time::now()).num_seconds().abs()+10) as u64,0);
			
			//sleep until then
			sleep(time_to_wait);
			
			let mut items_to_restock: Vec<Item> = Vec::new();
			//query that gets all item_keys in the restocking table, then selects those items from the inventory table
		    match self.restocking_database.results_from_database(
		    	format!("SELECT restocking.item_key,{0} FROM restocking, inventory where restocking.item_key=inventory.item_key",Item::field_names()),&mut items_to_restock) 
		    {
		    	None => {},
		    	Some(err) => { println!("Error accessing database in restocking thread: {:?}",err); continue;}//some error, report and restart thread
		    };
		    
		    let mut email_body = String::from(format!("We have {} requests for item restocks:\n",items_to_restock.len()));
		    for item in items_to_restock {
		    	email_body.push_str(format!("\t* {0} {1} ({2}) left in stock.\n",item.quantity, item.item_name, item.description).as_str());
		    }
		
			let email = EmailBuilder::new()
		                    .to(self.email_settings.restocker_email.as_str())
		                    .from("no-reply@InventoryManager")
		                    .body(email_body.as_str())
		                    .subject("Weekly restocking report")
		                    .build()
		                    .unwrap();			
		
			let result = mailer.send(email);
			println!("{:?}",result);
			
			self.restocking_database.alter_database(format!("truncate restocking"));
		}	
	}
}