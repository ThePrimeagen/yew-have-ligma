use std::sync::Arc;

use clap::Parser;
use tokio::{task::JoinHandle, sync::futures};

#[derive(Parser)]
#[clap()]
struct Args {
    spawns: usize,
    address: String,
    port: Option<u16>,
}

async fn request(args: Arc<Args>, count: usize) {
}

#[tokio::main]
async fn main() {
    let args = Arc::new(Args::parse());
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 1..=10 {
        let count = i * 100;
        for _ in 0..args.spawns {
            handles.push(tokio::spawn(request(args.clone(), count)));
        }
        futures::future::join_all(handles).await?;
    }
}


