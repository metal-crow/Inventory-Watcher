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
               		console.log(data);
               		new_item.innerHTML = "\
               		Name:<input placeholder=\"Item Name\" type=\"text\" id=\"item-name-"+i+"\" value=\""+data[i].item_name+"\">\
    				Description:<input type=\"text\" id=\"description-"+i+"\" value=\""+data[i].description+"\">\
    				Quantity:<input placeholder=\"Quantity\" type=\"text\" id=\"quantity-"+i+"\" value=\""+data[i].quantity+"\">\
    				<input style=\"display: none;\" type=\"number\" id=\"x_coord-"+i+"\" value="+data[i].x_coord+">\
    				<input style=\"display: none;\" type=\"number\" id=\"y_coord-"+i+"\" value="+data[i].y_coord+">\
    				<input style=\"display: none;\" type=\"number\" id=\"width-"+i+"\" value="+data[i].width+">\
    				<input style=\"display: none;\" type=\"number\" id=\"height-"+i+"\" value="+data[i].height+">\
    				<button id=\"find-item\" onclick=\"itemFind("+i+")\">Show Item Location</button>\
    				<button id=\"edit-item\" onclick=\"itemUpdate("+i+","+data[i].item_key+")\">Edit Item</button>\
    				";
                	list_parent.appendChild(new_item);
               }
             } else {
               console.log(e.responseText);
             }
           }
          }
         )
}

function itemFind(item_num) {
	//unhide reserch room image
	
	//load imgAreaSelect
	var x1 = document.getElementById("xcoord-"+item_num).value;
	var y1 = document.getElementById("ycoord-"+item_num).value;
	var x2 = x1 + document.getElementById("width-"+item_num).value;
	var y2 = y1 + document.getElementById("height-"+item_num).value;
	$('#img').imgAreaSelect({ x1: x1, y1: y1, x2: x2, y2: y2 });
	
	//for imgAreaSelect, onSelectEnd update the hidden field values
	
	//when close button clicked, hide research room image
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
             } else {
               console.log(e.responseText)
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
               console.log(e.responseText)
             }
           }
          }
         )
}