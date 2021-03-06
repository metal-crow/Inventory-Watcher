function request(args) {
  // route, params, method, callback
  args.method = args.method || 'POST'
  var params = JSON.stringify(args.item)
  var req = new XMLHttpRequest()
  var url = "http://localhost"  // change to location.origin in prod
  req.onreadystatechange = function() {
    if (req.readyState == XMLHttpRequest.DONE) {
        args.callback(req)
    }
  }
  req.open(args.method, url + args.route)
  req.send(params)
}

document.querySelector('#search-bar').onkeypress = function(e) {
  if (!e) e = window.event
  var keyCode = e.keyCode || e.which
  if (keyCode == '13'){
    var query = document.querySelector('#search-bar').value
    itemSearch(query)
  }
}

function itemSearch(item) {
  // Expects item to be a json string
  // {"item_name_or_description":"test"}
  document.querySelector('#search-bar').disabled = true
  request({route: '/ItemSearch',
           item: {item_name_or_description: item},
           callback: function(e) {
             // Do stuff with what it returns
             document.querySelector('#search-bar').disabled = false;
             if(e.status == 200){
               var data = JSON.parse(e.response);
               //clear parent
               var list_parent = document.getElementById('found-items');
			   while(list_parent.hasChildNodes()) {
				    list_parent.removeChild(list_parent.firstChild);
			   }
               //populate parent with data
               for(var i=0;i<data.length;i++) {
               		var new_item = document.createElement('div');
               		new_item.id = "item"+i;
               		new_item.innerHTML = "\
               		Name:<input placeholder=\"Item Name\" type=\"text\" id=\"item-name-"+i+"\" value=\""+data[i].item_name+"\">\
    				Description:<textarea rows=\"2\" cols=\"50\" type=\"text\" id=\"description-"+i+"\">"+data[i].description+"</textarea>\
    				Quantity:<input placeholder=\"Quantity\" type=\"text\" id=\"quantity-"+i+"\" value=\""+data[i].quantity+"\">\
    				<input style=\"display: none;\" type=\"number\" id=\"x_coord-"+i+"\" value="+data[i].x_coord+">\
    				<input style=\"display: none;\" type=\"number\" id=\"y_coord-"+i+"\" value="+data[i].y_coord+">\
    				<input style=\"display: none;\" type=\"number\" id=\"width-"+i+"\" value="+data[i].width+">\
    				<input style=\"display: none;\" type=\"number\" id=\"height-"+i+"\" value="+data[i].height+">\
    				<button id=\"find-item\" onclick=\"itemFind("+i+")\">Show Item Location</button>\
    				<button id=\"edit-item\" onclick=\"itemUpdate("+i+","+data[i].item_key+")\">Save Changes</button>\
    				<button id=\"alert-item\" onclick=\"itemLowAlert("+data[i].item_key+")\">Restock Item</button>\
    				<button id=\"delete-item\" onclick=\"itemDelete("+data[i].item_key+")\">Delete Item</button>\
    				";
                	list_parent.appendChild(new_item);
               }
               if(data.length==0) {
               		list_parent.innerHTML = "No Items Found";
               }
             } else {
               console.log(e.responseText);
             }
           }
          }
         )
}

function itemLowAlert(item_key){
	  request({route: '/ItemAlert',
           item: {item_key : item_key},
           callback: function(e) {
             console.log(e)
             if(e.status == 200){
               console.log('success')
             } else {
               alert(e.responseText)
             }
           }
          }
         )
}

function itemFind(item_num) {
	//unhide research room image (actually create it to allow click thorugh)
	var imgs_parent_div = document.getElementById('room_photo_div');
	
	var room_photo_img = document.createElement('img');
	room_photo_img.id = "room_photo";
	room_photo_img.src = "/public/research_room.png"
	imgs_parent_div.appendChild(room_photo_img);
	
	var close_photo_img = document.createElement('img');
	close_photo_img.id = "close_room_photo";
	close_photo_img.src = "/public/close_icon.png"
	close_photo_img.addEventListener("click", removeItemFind);
	imgs_parent_div.appendChild(close_photo_img);
	
	//load imgAreaSelect
	var x1 = parseInt(document.getElementById("x_coord-"+item_num).value);
	var y1 = parseInt(document.getElementById("y_coord-"+item_num).value);
	var x2 = parseInt(x1) + parseInt(document.getElementById("width-"+item_num).value);
	var y2 = parseInt(y1) + parseInt(document.getElementById("height-"+item_num).value);
	$('#room_photo').imgAreaSelect({ 
		x1: x1, y1: y1, x2: x2, y2: y2, 
		handles: true, 
		onSelectEnd: function (img, selection) {
	        	document.getElementById("x_coord-"+item_num).value = selection.x1;
				document.getElementById("y_coord-"+item_num).value = selection.y1;
				document.getElementById("width-"+item_num).value = selection.width;
				document.getElementById("height-"+item_num).value = selection.height;
	    } 
	});
}

//when close button clicked, hide research room image
function removeItemFind() {
	$('#room_photo').imgAreaSelect({ remove:true });
	var imgs_parent_div = document.getElementById('room_photo_div');
	while(imgs_parent_div.hasChildNodes()) {
		imgs_parent_div.removeChild(imgs_parent_div.firstChild);
	}
}

document.getElementById("add-new-item").addEventListener('click', function(){
    itemAdd({
    item_name: document.getElementById("item-name").value,
    description: document.getElementById("description").value,
    quantity: document.getElementById("quantity").value,
    x_coord: document.getElementById("x_coord").value,
    y_coord: document.getElementById("y_coord").value,
    width: document.getElementById("width").value,
    height: document.getElementById("height").value,
    })
});

function itemAdd(item) {
  // Expects item to be a json string
  request({route: '/ItemAdd',
           item: item,
           callback: function(e) {
             console.log(e)
             if(e.status == 200){
               console.log('success')
               document.getElementById("item-name").value = "";
    		   document.getElementById("description").value = "";
			   document.getElementById("quantity").value = "";               
             } else {
               alert(e.responseText)
             }
           }
          }
         )
}

function itemUpdate(item_num,item_key) {
  var item = {    
    item_key: item_key,
    item_name: document.getElementById("item-name-"+item_num).value,
    description: document.getElementById("description-"+item_num).value,
    quantity: document.getElementById("quantity-"+item_num).value,
    x_coord: document.getElementById("x_coord-"+item_num).value,
    y_coord: document.getElementById("y_coord-"+item_num).value,
    width: document.getElementById("width-"+item_num).value,
    height: document.getElementById("height-"+item_num).value,
  };
  // Expects item to be a json string
  request({route: '/ItemUpdate',
           item: item,
           callback: function(e) {
             console.log(e)
             if(e.status == 200){
               console.log('success')
             } else {
               alert(e.responseText)
             }
           }
          }
         )
}

function itemDelete(item_key) {
  request({route: '/ItemDelete',
           item: {item_key: item_key},
           callback: function(e) {
             console.log(e)
             if(e.status == 200){
               console.log('success')
             } else {
               alert(e.responseText)
             }
           }
          }
         )
}
