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
Server API:
 * Request info about item
 * Add item to inventory
 * **Search for item based on name or description**
Client:
 * Get and display info about item 
 
###ToDo 
behind webauth  