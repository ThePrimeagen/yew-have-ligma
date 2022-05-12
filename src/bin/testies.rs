use std::{sync::{Arc, atomic::{AtomicUsize, Ordering}}, time::{Duration, Instant}};

use clap::Parser;
use ::futures::{stream::FuturesUnordered, StreamExt};
use once_cell::sync::Lazy;
use tokio_util::task::LocalPoolHandle;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

static LOCAL_POOL: Lazy<LocalPoolHandle> = Lazy::new(|| LocalPoolHandle::new(num_cpus::get() * 2));

#[derive(Parser)]
#[clap()]
struct Args {
    #[clap(short = 'd', long = "depth", default_value= "3")]
    depth: usize,

    #[clap(short = 'g', long = "girth", default_value= "69")]
    girth: usize,

    #[clap(short = 'c', long = "count", default_value= "1000")]
    count: usize,

    #[clap(short = 'a', long = "address", default_value= "0.0.0.0")]
    address: String,

    #[clap(short = 'p', long = "port", default_value = "42069")]
    port: u16,

    #[clap(short = 't', long = "threads", default_value = "8")]
    threads: usize,

    #[clap(short = 'm', long = "max_conn", default_value = "100")]
    max_conn: usize,
}

#[derive(Debug)]
struct Testing {
    current: AtomicUsize,
    error: AtomicUsize,
    success: AtomicUsize,
}

async fn request(args: Arc<Args>) -> Result<String, ()> {
    let handle = match LOCAL_POOL
        .spawn_pinned(move || async move {

            let address = format!("http://{}:{}/render/{}/{}", args.address, args.port, args.depth, args.girth);

            let result = match reqwest::get(address).await {
                Ok(r) => r,
                Err(_) => {
                    return Err(())
                },
            };

            let headers = result.headers();
            let time_taken: String = match headers["time-taken"].to_str() {
                Ok(t) => t,
                _ => "-1",
            }.to_string();

            match result.text().await {
                Ok(_) => {},
                Err(_) => {
                    return Err(())
                },
            };

            return Ok(time_taken);
        }).await {
            Ok(t) => t,
            Err(_) => {
                return Err(())
            }
        };

    return handle;
}

async fn wait_for_availabilities(args: &Arc<Args>, testing: Arc<Testing>) {
    while testing.current.load(Ordering::Relaxed) > args.max_conn {
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
}

async fn request_inner_loop(args: Arc<Args>, testing: Arc<Testing>) -> String {
    wait_for_availabilities(&args, testing.clone()).await;

    testing.current.fetch_add(1, Ordering::Relaxed);
    let time_taken: String;

    loop {
        match request(args.clone()).await {
            Ok(tt) => {
                time_taken = tt;
                testing.success.fetch_add(1, Ordering::Relaxed);
                break;
            },
            _ => {
                testing.error.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    testing.current.fetch_sub(1, Ordering::Relaxed);
    return time_taken;
}

async fn request_loop(args: Arc<Args>, count: usize, testing: Arc<Testing>) {
    let futures: FuturesUnordered<_> = (0..count)
        .map(|_| request_inner_loop(args.clone(), testing.clone()))
        .collect();

    let timings: Vec<String> = futures.collect().await;

    println!("timing {}", timings.join(","));
}

#[tokio::main]
async fn main() {
    let args = Arc::new(Args::parse());
    let count = args.count / args.threads;
    let testing = Arc::new(Testing {
        current: AtomicUsize::new(0),
        error: AtomicUsize::new(0),
        success: AtomicUsize::new(0),
    });
    let now = Instant::now();

    let inner_testing = testing.clone();
    futures::future::join_all((0..args.threads)
        .map(move |_| {
            return tokio::spawn(request_loop(args.clone(), count, inner_testing.clone()));
        })).await;

    let duration = now.elapsed().as_secs() as usize;
    let success = testing.success.load(Ordering::Relaxed);
    let error = testing.error.load(Ordering::Relaxed);
    println!("success {},{}",
        success,
        success / duration);

    println!("error {},{}",
        error,
        error / duration);
}


