use react_vs_wasm_yew::app::{ReactSucks, AppProps};
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/render/{depth}/{girth}")]
async fn greet(depth: web::Path<(usize, usize)>) -> impl Responder {
    let d = depth.0;
    let g = depth.1;

    let renderer = yew::ServerRenderer::<ReactSucks>::with_props(AppProps {
        size: g,
        depth: d,
        interval: 3
    });

    let rendered = renderer.render().await;

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered);
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
    })
    .bind(("0.0.0.0", 42069))?
    .run()
    .await
}
