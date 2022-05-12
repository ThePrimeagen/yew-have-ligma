use actix_files::Files;
use clap::Parser;
use react_vs_wasm_yew::app::{ReactSucks, AppProps};
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/render/{depth}/{girth}")]
async fn greet(data: web::Data<RenderingData>, depth: web::Path<(usize, usize)>) -> impl Responder {
    let d = depth.0;
    let g = depth.1;

    let renderer = yew::ServerRenderer::<ReactSucks>::with_props(AppProps {
        size: g,
        depth: d,
        interval: 3
    });

    let rendered = renderer.render().await;
    let rendered = data.index_file.replace("__REPLACE_ME_DADDY__", &rendered);

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered);
}

#[derive(Parser)]
#[clap()]
struct Args {
    index_file: String
}

#[derive(Clone)]
struct RenderingData {
    index_file: String
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let index_file: Vec<u8> = std::fs::read(args.index_file)?;
    let index_file: String = std::str::from_utf8(&index_file)
        .expect("expected utf8 index file")
        .to_string();

    let data = web::Data::new(RenderingData {
        index_file,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(greet)
            .service(Files::new("/", "./dist"))
    })
    .max_connection_rate(10000)
    .worker_max_blocking_threads(10000)
    .max_connections(10000)
    .workers(num_cpus::get() * 2)
    .bind(("0.0.0.0", 42069))?
    .run()
    .await
}
