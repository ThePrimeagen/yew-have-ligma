use clap::Parser;
use reqwest::Client;
use std::{sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
}, time::Duration};
use tokio::{sync::Semaphore, time::Instant};

#[derive(Parser)]
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

    #[clap(short = 'm', long = "max_conn", default_value = "100")]
    max_conn: usize,
}

#[derive(Default)]
struct Stats {
    error: AtomicUsize,
    success: AtomicUsize,
    total_time_taken: AtomicUsize,
}

async fn send_request(client: Client, url: &str, stats: &Stats) {
    let resp = match reqwest::get(url).await {
        Ok(r) => r,
        Err(_) => {
            stats.error.fetch_add(1, Ordering::Relaxed);
            return;
        }
    };

    let headers = resp.headers();
    let time_taken = headers.get("time-taken");

    if time_taken.is_none() {
        return;
    }

    let time_taken = time_taken.unwrap().to_str().map(|x| {
        return str::parse::<usize>(x).unwrap_or(0);
    }).unwrap_or(0);

    stats.success.fetch_add(1, Ordering::Relaxed);
    stats.total_time_taken.fetch_add(time_taken, Ordering::Relaxed);
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = Client::new();
    let stats = Arc::new(Stats::default());

    let semaphore = Arc::new(Semaphore::new(args.max_conn));

    let url = Arc::new(format!("http://{}:{}/render/{}/{}", args.address, args.port, args.depth, args.girth));
    let mut handles = vec![];

    let now = Instant::now();
    for _ in 0..args.count {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let client = client.clone();
        let stats = stats.clone();
        let url = url.clone();

        handles.push(tokio::spawn(async move {
            send_request(client, &url, &stats).await;
            drop(permit);
        }));
    }

    tokio::spawn(async move {
        while semaphore.available_permits() != args.max_conn {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }).await.unwrap_or(());

    let total_time = now.elapsed().as_secs();
    let success = stats.success.load(Ordering::Relaxed);
    let error = stats.error.load(Ordering::Relaxed);

    println!("total_time: {} success {} errors {}", total_time, success, error);
    let rps = args.count as u64 / total_time;
    let average_ssr = stats.total_time_taken.load(Ordering::Relaxed) / success;

    println!("rps: {}, average_ssr: {}", rps, average_ssr);
}

