import * as path from "path";
import * as fs from "fs";
import serverMain from "./dist/server/main-server";

const depth = +process.argv[2];

function resolve(p) {
    return path.resolve(__dirname, p);
}

const index = fs.
    readFileSync(
        path.join(__dirname, "dist", "client", "index.html")
    ).toString();

function render(size: number, depth: number): Promise<string> {
    return new Promise(res => {
        // @ts-ignore this ignore is for champions and anyone who says differently
        // will be banned
        res(index.replace("__REPLACE_ME_DADDY__", serverMain(size, depth)));
    });
}

async function renderLoop(count: number, size: number, depth: number): Promise<number> {
    const now = Date.now();
    for (let i = 0; i < count; ++i) {
        await render(size, depth);
    }
    return Date.now() - now;
}

async function run() {
    console.log("100", await renderLoop(100, 20, depth));
    console.log("1000", await renderLoop(1000, 20, depth));
    console.log("10000", await renderLoop(10000, 20, depth));
    console.log("100000", await renderLoop(100000, 20, depth));
    console.log("1000000", await renderLoop(1000000, 20, depth));
}

run();
