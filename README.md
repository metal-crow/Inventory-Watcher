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
 
###Features
Server API (Rust):
 * `/ItemInfo`: Request info about item, given primary key
 * `/ItemSearch`: Search for item based on partial name or partial description 
 * `/ItemAdd`: Add item to inventory
 * `/ItemUpdate`: Update item info, given primary key
 * `/ItemFind`: Find item: send command to hardware
 * `/`: Serve client html/js view
 
Client (Javascript/HTML):
 * Search for items
 * Get and display info about item 
 * Request item location be shown
 * Update item stats
 
Hardware (Laser, RasPi):
 * Store coordinates with item in Database
 * Have laser attached to gimble and RasPi
 * When Pi receives data over network, point laser at received coords
 
###SQL Schema
Item:
 * item_name: String[Primary Key]  
 * quantity: u32  
 * description: String[non-null, can be empty]  
 * x_coord: u32  
 * y_coord: u32  
  
###Settings.ini
\[MySQL\]:  
 * user
 * password
 * database_name
 * ip\_or\_hostname
 * port
 
\[RasPi\]: 
 * rasPi\_ip\_or\_host
 
###ToDo 
behind webauth  