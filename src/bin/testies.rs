use std::{sync::Arc, time::Duration};

use clap::Parser;
use ::futures::{stream::FuturesUnordered, StreamExt};
use once_cell::sync::Lazy;
use tokio_util::task::LocalPoolHandle;
use serde::Deserialize;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

static LOCAL_POOL: Lazy<LocalPoolHandle> = Lazy::new(|| LocalPoolHandle::new(8));

#[derive(Parser)]
#[clap()]
struct Args {
    #[clap(short = 'd', long = "depth", default_value= "3")]
    depth: usize,

    #[clap(short = 'g', long = "girth", default_value= "69")]
    girth: usize,

    #[clap(short = 'c', long = "clap", default_value= "1000")]
    count: usize,

    #[clap(short = 'a', long = "address", default_value= "0.0.0.0")]
    address: String,

    #[clap(short = 'p', long = "port", default_value = "42069")]
    port: u16,

    #[clap(short = 'r', long = "request-spacing", default_value = "5")]
    request_spacing: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct Response {
}

async fn request(args: Arc<Args>, position: usize) {
    LOCAL_POOL
        .spawn_pinned(move || async move {
            let sleep_duration = (position * args.request_spacing) as u64;
            tokio::time::sleep(Duration::from_millis(sleep_duration)).await;

            let address = format!("http://{}:{}/render/{}/{}", args.address, args.port, args.depth, args.girth);

            match reqwest::get(address)
                .await
                .unwrap()
                .text()
                .await {
                Ok(_) => { },
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }

        }).await.unwrap();
}

async fn request_loop(args: Arc<Args>, count: usize) {
    let futures: FuturesUnordered<_> = (0..count)
        .enumerate()
        .map(|(i, _)| request(args.clone(), i))
        .collect();

    let _: Vec<_> = futures.collect().await;
}

#[tokio::main]
async fn main() {
    let args = Arc::new(Args::parse());
    let count = args.count / 8;

    futures::future::join_all((0..8)
        .map(|_| tokio::spawn(request_loop(args.clone(), count)))).await;
}


