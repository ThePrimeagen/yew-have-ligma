use react_vs_wasm_yew::app::{AppProps, ReactSucks};

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio_util::task::LocalPoolHandle;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

static LOCAL_POOL: Lazy<LocalPoolHandle> = Lazy::new(|| LocalPoolHandle::new(num_cpus::get() * 2));

async fn render(size: usize, depth: usize, index_file: Arc<str>) {
    LOCAL_POOL
        .spawn_pinned(move || async move {
            let renderer = yew::ServerRenderer::<ReactSucks>::with_props(AppProps {
                size,
                depth,
                interval: 3,
            });

            let rendered = renderer.render().await;

            let _s = index_file.replace("__REPLACE_ME__", &rendered);
        })
        .await
        .unwrap()
}

async fn render_loop(count: usize, size: usize, depth: usize, index_file: Arc<str>) -> u128 {
    let now = std::time::Instant::now();
    let futures: FuturesUnordered<_> = (0..count)
        .map(|_| render(size, depth, index_file.clone()))
        .collect();

    let _: Vec<_> = futures.collect().await;

    now.elapsed().as_millis()
}

#[tokio::main]
async fn main() {
    let depth: usize = str::parse(&std::env::args().nth(1).expect("must")).expect("must");
    let index_file: Arc<str> = std::fs::read_to_string("./index.html")
        .expect("to work")
        .into();

    for i in 2..6 {
        let count = 10_usize.pow(i);

        let content = render_loop(count, 20, depth, index_file.clone()).await;
        println!("{:>7}: {}ms", count, content)
    }
}

