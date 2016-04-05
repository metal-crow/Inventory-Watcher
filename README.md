##CSH Research Room Inventory Watcher

An inventory manager for the CSH research room. Keeps track of parts and their current quantity.  
Also allows locating parts via an automated laser pointer.  
Used this as an excuse to learn Rust.  

Windows Setup:  
 * install Rust x64
 * install MSYS2
  * install mingw-w64-x86_64-gcc, add C:\MSYS64\mingw64\bin to PATH
  * install mingw-w64-x86_64-openssl
 * install MySQL
 
Linux Setup:  
 * Change cargo.toml to only have `mysql="*"`, instead of the other mysql stuff
 * You must install openssl from the package manager, see https://github.com/sfackler/rust-openssl
 * Also don't forget to install gcc, rust, and an sql database 
 
###Features
Server API (Rust):
 * `/ItemSearch`: Search for item based on partial name or partial description 
 * `/ItemAdd`: Add item to inventory
 * `/ItemUpdate`: Update item info, given primary key
 * `/ItemDelete`: Delete item, given primary key
 * `/ItemAlert`: Send an email that the given item needs to be restocked
 * `/`: Serve client html/js view
 * `/public`: Serve everything in the selected folder as static
 
Client (Javascript/HTML):
 * Search for items
 * Edit/Add items
  * Select item's location in photo via javascript click-and-drag highlighting of image
 * Get and display info about item: Done via highlighting area in picture of room
 
###SQL Schema
Item:
 * item_key: u64[non-null, unsigned, auto incremented, primary key]
 * item_name: String(45)[non-null, unique]  
 * quantity: u32[non-null, unsigned, no default]  
 * description: String(45)[non-null, default '']  
 * x_coord: u32[unsigned, default 0]  
 * y_coord: u32[unsigned, default 0]    
 * width: u32[unsigned, default 0]    
 * height: u32[unsigned, default 0]    
  
###Settings.ini
\[MySQL\]:  
 * user
 * password
 * database_name
 * ip\_or\_hostname
 * port
 
\[Server\]:
 * dns_name
 * restock_email
 
###Libraries
Rust:
 * iron
 * rustc-serialize
 * router
 * rust-ini
 * staticfile
 * mount
 
HTML/JS:  
 * JQuery
 * ImgAreaSelect