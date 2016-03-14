##Know Rust? You'll hate this!

An inventory manager for the CSH research room. Keeps track of parts and their current quantity.  
Used this as an excuse to learn Rust.  

Windows Setup:  
 * install Rust x64
 * install MSYS2
  * mingw-w64-x86_64-gcc, add C:\MSYS64\mingw64\bin to PATH
  * install mingw-w64-x86_64-openssl
 * MySQL
 
###Features
Server API (Rust):
 * Request info about item, given primary key
 * Search for item based on partial name or partial description 
 * Add item to inventory
 * Update item info, given primary key
 * Find item: send command to hardware
 
Client (Javascript/HTML):
 * Get and display info about item 
 
Hardware (Laser, Arduino):
 * Store coordinates with item in Database
 * Have laser attached to gible and arduino
 * Point laser at selected item's coords
 
###ToDo 
behind webauth  