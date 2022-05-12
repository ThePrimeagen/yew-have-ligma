import * as path from "path";
import * as fs from "fs";
import * as express from "express";
import serverMain from "./dist/server/main-server";

console.log(serverMain);

const app = express();

function resolve(p) {
    return path.resolve(__dirname, p);
}

app.use(
  require('serve-static')(resolve('dist/client'), {
    index: false
  })
)

const index = fs.
    readFileSync(
        path.join(__dirname, "dist", "client", "index.html")
    ).toString();


function getMicroTime(): number {
    var hrTime = process.hrtime()
    return hrTime[0] * 1000000 + hrTime[1] / 1000;
}

app.use("/:size/:depth", function(req, res) {
    const now = getMicroTime();
    const size = +req.params.size;
    const depth = +req.params.depth;

    // @ts-ignore this ignore is for champions and anyone who says differently
    // will be banned
    const html = index.replace("__REPLACE_ME_DADDY__", serverMain(size, depth));
    res.
        setHeader("time-taken", getMicroTime() - now).
        status(200).
        set({"Content-Type": "text/html"}).end(html)
});

app.listen(42069, () => {
    console.log('http://localhost:42069')
})
