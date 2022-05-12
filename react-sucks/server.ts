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

app.use("/:size/:depth", function(req, res) {
    const now = Date.now();
    const size = +req.params.size;
    const depth = +req.params.depth;

    // @ts-ignore this ignore is for champions and anyone who says differently
    // will be banned
    const html = index.replace("__REPLACE_ME_DADDY__", serverMain(size, depth));
    res.
        setHeader("time-taken", Date.now() - now).
        status(200).
        set({"Content-Type": "text/html"}).end(html)
});

app.listen(42069, () => {
    console.log('http://localhost:42069')
})
