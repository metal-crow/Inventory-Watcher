function request(args) {
  // route, params, method, callback
  args.method = args.method || 'POST'
  var params = JSON.stringify(args.item)
  var req = new XMLHttpRequest()
  var url = "http://localhost:3000"  // change to location.origin in prod
  req.onreadystatechange = function() {
    if (req.readyState == XMLHttpRequest.DONE) {
        args.callback(req)
    }
  }
  req.open(args.method, url + args.route)
  req.send(params)
}

function itemInfo(item) {
  // Expects item to be a json string
  request({route: '/ItemInfo',
           item: {item_name: item},
           method: "GET",
           callback: function(e) {
             // Do stuff with what it returns
             console.info(e)
             if(e.status == 200){
               console.log('success')
             } else {
               console.log(e.responseText)
             }
           }
          }
         )
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
               console.log(e.response);
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
               		Name:<input placeholder=\"Item Name\" type=\"text\" id=\"item-name-"+i+"\" value="+data[i].item_name+">\
    				Description:<input type=\"text\" id=\"description-"+i+"\" value="+data[i].description+">\
    				Quantity:<input placeholder=\"Quantity\" type=\"text\" id=\"quantity-"+i+"\" value="+data[i].quantity+">\
    				<button id=\"add-new-item\" onclick=\"itemUpdate("+i+")\">Edit Item</button>\
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

document.getElementById("add-new-item").addEventListener('click', function(){
    itemAdd({
    item_name: document.getElementById("item-name").value,
    description: document.getElementById("description").value,
    quantity: document.getElementById("quantity").value
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

function itemUpdate(item_num) {
  var item = {    
    item_name: document.getElementById("item-name-"+item_num).value,
    description: document.getElementById("description-"+item_num).value,
    quantity: document.getElementById("quantity-"+item_num).value
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

// function itemFind(item) {
// TODO: This is route is subject to change, so we'll deal with it later
// }