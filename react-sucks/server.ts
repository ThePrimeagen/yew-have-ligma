import express from "express";
import serverMain from "./src/main-server";

const app = express();

app.use("*", function(req, res) {
    console.log(req.query)
    const html = serverMain(3, 3);
    res.status(200).set({ 'Content-Type': 'text/html' }).end(html)
});

app.listen(42068, () => {
    console.log('http://localhost:3000')
})
