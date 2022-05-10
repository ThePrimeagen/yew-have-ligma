"use strict";
exports.__esModule = true;
var path = require("path");
var fs = require("fs");
var express = require("express");
var main_server_1 = require("./dist/server/main-server");
console.log(main_server_1["default"]);
var app = express();
function resolve(p) {
    return path.resolve(__dirname, p);
}
app.use(require('serve-static')(resolve('dist/client'), {
    index: false
}));
var index = fs.
    readFileSync(path.join(__dirname, "dist", "client", "index.html")).toString();
app.use("/:size/:depth", function (req, res) {
    var size = +req.params.size;
    var depth = +req.params.depth;
    // @ts-ignore this ignore is for champions and anyone who says differently
    // will be banned
    var html = index.replace("__REPLACE_ME_DADDY__", (0, main_server_1["default"])(size, depth));
    res.status(200).set({ "Content-Type": "text/html" }).end(html);
});
app.listen(42068, function () {
    console.log('http://localhost:42068');
});
