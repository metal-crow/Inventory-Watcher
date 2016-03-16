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
 
###Features
Server API (Rust):
 * Request info about item, given primary key
 * Search for item based on partial name or partial description 
 * Add item to inventory
 * Update item info, given primary key
 * Find item: send command to hardware
 
Client (Javascript/HTML):
 * Search for items
 * Get and display info about item 
 * Request item location be shown
 * Update item stats
 
Hardware (Laser, RasPi):
 * Store coordinates with item in Database
 * Have laser attached to gimble and RasPi
 * When Pi receives data over network, point laser at received coords
 
###ToDo 
behind webauth  