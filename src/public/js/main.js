function updateProgress(e) {
  console.log(e)
}

function request(args) {
  // route, params, method, callback
  args.method = args.method || 'POST'
  args.item = JSON.stringify(args.item)
  var req = new XMLHttpRequest()
  var url = "http://localhost:3000"  // change to location.origin in prod
  req.addEventListener("progress", updateProgress)
  req.addEventListener("load", args.callback)
  req.addEventListener("error", args.callback)
  req.addEventListener("abort", args.callback)
  req.open(args.method, url + args.route)
  req.send(args.params)
}

function itemInfo(item) {
  // Expects item to be a json string
  request({route: '/ItemInfo',
           item: {item_name: item},
           method: "GET",
           callback: function(e) {
             // Do stuff with what it returns
             console.info(e)
             if(e.type == "load"){
               console.log('success')
             } else {
               console.log('error')
             }
           }
          }
         )
}

function itemSearch(item) {
  // Expects item to be a json string
  // {"item_name_or_description":"test"}
  document.querySelector('#search-bar').disabled = true
  request({route: '/ItemSearch',
           item: {item_name_or_description: item},
           method: "GET",
           callback: function(e) {
             // Do stuff with what it returns
             document.querySelector('#search-bar').disabled = false
             if(e.type == "load"){
               console.log('success')
               document.querySelector('item-name').value = e.data.item_name
               document.querySelector('description').value = e.data.description
               document.querySelector('quantity').value = e.data.quantity
             } else {
               console.log('error')
             }
           }
          }
         )
}

function itemAdd(item) {
  // Expects item to be a json string
  request({route: '/ItemAdd',
           item: item,
           callback: function(e) {
             console.log(e)
             if(e.type == "load"){
               console.log('success')
             } else {
               console.log('error')
             }
           }
          }
         )
}

function itemUpdate(item) {
  // Expects item to be a json string
  request({route: '/ItemUpdate',
           item: item,
           callback: function(e) {
             console.log(e)
             if(e.type == "load"){
               console.log('success')
             } else {
               console.log('error')
             }
           }
          }
         )
}

// function itemFind(item) {
// TODO: This is route is subject to change, so we'll deal with it later
// }

document.querySelector('#search-bar').onkeypress = function(e) {
  if (!e) e = window.event
  var keyCode = e.keyCode || e.which
  if (keyCode == '13'){
    var query = document.querySelector('#search-bar').value
    itemSearch(query)
  }
}
