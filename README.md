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
 * `/ItemFind`: Find item: TODO either return photo with the area highlighted or give the photos from `/`, and return photo # and location to highlight from js
 * `/`: Serve client html/js view
 * `/public`: Serve everything in the selected folder as static TODO
 
Client (Javascript/HTML):
 * Search for items
 * Get and display info about item: Done via highlighting area in picture of room [Should this be done via qr codes and cv?]
 * Request item location be shown
 * Update item stats
 
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
 
###ToDo 
behind webauth  