function updateProgress(e) {
  console.log(e)
}

function transferFailed(e) {
  console.warn(e)
}

function transferCanceled(e) {
  console.warn(e)
}

function request(args) {
  // route, params, method, callback
  args.method = args.method || 'POST'
  args.item = JSON.stringify(args.item)
  var req = new XMLHttpRequest()
  var url = "http://localhost:3000"  // change to location.origin in prod
  req.addEventListener("progress", updateProgress)
  req.addEventListener("load", args.callback)
  req.addEventListener("error", transferFailed)
  req.addEventListener("abort", transferCanceled)
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
           }
          })
}

function itemSearch(item) {
  // Expects item to be a json string
  // {"item_name_or_description":"test"}
  request({route: '/ItemSearch',
           item: {item_name_or_description: item},
           method: "GET",
           callback: function(e) {
             // Do stuff with what it returns
             console.info(e)
           }
          })
}

function itemAdd(item) {
  // Expects item to be a json string
  request({route: '/ItemAdd',
           item: item,
           callback: function(e) {
             console.log(e)
           }
          })
}

function itemUpdate(item) {
  // Expects item to be a json string
  request({route: '/ItemUpdate',
           item: item,
           callback: function(e) {
             console.log(e)
           }
          })
}

// function itemFind(item) {
// TODO: This is route is subject to change, so we'll deal with it later
// }
