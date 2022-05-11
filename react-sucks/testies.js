"use strict";
exports.__esModule = true;
var path = require("path");
var fs = require("fs");
var main_server_1 = require("./dist/server/main-server");
function resolve(p) {
    return path.resolve(__dirname, p);
}
var index = fs.
    readFileSync(path.join(__dirname, "dist", "client", "index.html")).toString();
function render(size, depth) {
    // @ts-ignore this ignore is for champions and anyone who says differently
    // will be banned
    return index.replace("__REPLACE_ME_DADDY__", (0, main_server_1["default"])(size, depth));
}
function renderLoop(count, size, depth) {
    var now = Date.now();
    for (var i = 0; i < count; ++i) {
        render(size, depth);
    }
    return Date.now() - now;
}
var depth = +process.argv[2];
if (!depth) {
    console.error("provide a depth please ./prog <depth>");
    process.exit(1);
}
console.log("100", renderLoop(100, 20, depth));
console.log("1000", renderLoop(1000, 20, depth));
console.log("10000", renderLoop(10000, 20, depth));
console.log("100000", renderLoop(100000, 20, depth));
console.log("1000000", renderLoop(1000000, 20, depth));
