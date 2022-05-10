"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
var path = require("path");
var fs = require("fs");
var main_server_1 = require("./dist/server/main-server");
var depth = +process.argv[2];
function resolve(p) {
    return path.resolve(__dirname, p);
}
var index = fs.
    readFileSync(path.join(__dirname, "dist", "client", "index.html")).toString();
function render(size, depth) {
    return new Promise(function (res) {
        // @ts-ignore this ignore is for champions and anyone who says differently
        // will be banned
        res(index.replace("__REPLACE_ME_DADDY__", (0, main_server_1["default"])(size, depth)));
    });
}
function renderLoop(count, size, depth) {
    return __awaiter(this, void 0, void 0, function () {
        var now, i;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    now = Date.now();
                    i = 0;
                    _a.label = 1;
                case 1:
                    if (!(i < count)) return [3 /*break*/, 4];
                    return [4 /*yield*/, render(size, depth)];
                case 2:
                    _a.sent();
                    _a.label = 3;
                case 3:
                    ++i;
                    return [3 /*break*/, 1];
                case 4: return [2 /*return*/, Date.now() - now];
            }
        });
    });
}
function run() {
    return __awaiter(this, void 0, void 0, function () {
        var _a, _b, _c, _d, _e, _f, _g, _h, _j, _k, _l, _m, _o, _p, _q;
        return __generator(this, function (_r) {
            switch (_r.label) {
                case 0:
                    _b = (_a = console).log;
                    _c = ["100"];
                    return [4 /*yield*/, renderLoop(100, 20, depth)];
                case 1:
                    _b.apply(_a, _c.concat([_r.sent()]));
                    _e = (_d = console).log;
                    _f = ["1000"];
                    return [4 /*yield*/, renderLoop(1000, 20, depth)];
                case 2:
                    _e.apply(_d, _f.concat([_r.sent()]));
                    _h = (_g = console).log;
                    _j = ["10000"];
                    return [4 /*yield*/, renderLoop(10000, 20, depth)];
                case 3:
                    _h.apply(_g, _j.concat([_r.sent()]));
                    _l = (_k = console).log;
                    _m = ["100000"];
                    return [4 /*yield*/, renderLoop(100000, 20, depth)];
                case 4:
                    _l.apply(_k, _m.concat([_r.sent()]));
                    _p = (_o = console).log;
                    _q = ["1000000"];
                    return [4 /*yield*/, renderLoop(1000000, 20, depth)];
                case 5:
                    _p.apply(_o, _q.concat([_r.sent()]));
                    return [2 /*return*/];
            }
        });
    });
}
run();
