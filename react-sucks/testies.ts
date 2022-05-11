import * as path from "path";
import * as fs from "fs";
import serverMain from "./dist/server/main-server";

function resolve(p) {
    return path.resolve(__dirname, p);
}

const index = fs.
    readFileSync(
        path.join(__dirname, "dist", "client", "index.html")
    ).toString();

function render(size: number, depth: number): string {
    // @ts-ignore this ignore is for champions and anyone who says differently
    // will be banned
    return index.replace("__REPLACE_ME_DADDY__", serverMain(size, depth));
}

function renderLoop(count: number, size: number, depth: number): number {
    const now = Date.now();
    for (let i = 0; i < count; ++i) {
        render(size, depth);
    }
    return Date.now() - now;
}

console.log("100", renderLoop(100, 20, 2));
console.log("1000", renderLoop(1000, 20, 2));
console.log("10000", renderLoop(10000, 20, 2));
console.log("100000", renderLoop(100000, 20, 2));
console.log("1000000", renderLoop(1000000, 20, 2));
