use std::{time::Instant, sync::atomic::AtomicUsize};

use actix_files::Files;
use clap::Parser;
use react_vs_wasm_yew::app::{ReactSucks, AppProps};
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[get("/render/{depth}/{girth}")]
async fn greet(data: web::Data<RenderingData>, depth: web::Path<(usize, usize)>) -> impl Responder {
    data.current.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let now = Instant::now();
    let d = depth.0;
    let g = depth.1;

    let renderer = yew::ServerRenderer::<ReactSucks>::with_props(AppProps {
        size: g,
        depth: d,
        interval: 3
    });

    let rendered = renderer.render().await;
    let rendered = data.index_file.replace("__REPLACE_ME_DADDY__", &rendered);
    let current = data.current.load(std::sync::atomic::Ordering::Relaxed); // count up forever

    let resp = HttpResponse::Ok()
        .content_type("text/html")
        .insert_header(("time-taken", now.elapsed().as_micros().to_string()))
        .insert_header(("current", current.to_string()))
        .body(rendered);

    return resp;
}

#[derive(Parser)]
#[clap()]
struct Args {
    index_file: String
}

struct RenderingData {
    index_file: String,
    current: AtomicUsize,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let workers: usize = str::parse(&std::env::var("WORKERS").unwrap_or("1".to_string())).unwrap_or(1);
    let index_file: Vec<u8> = std::fs::read(args.index_file)?;
    let index_file: String = std::str::from_utf8(&index_file)
        .expect("expected utf8 index file")
        .to_string();

    let data = web::Data::new(RenderingData {
        index_file,
        current: AtomicUsize::new(0),
    });

    println!("workers {}", workers);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(greet)
            .service(Files::new("/", "./dist"))
    })
    .workers(num_cpus::get() * workers)
    .bind(("0.0.0.0", 42069))?
    .run()
    .await
}
