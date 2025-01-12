use std::hint::black_box;

use futures::StreamExt;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    println!("Starting reqwest_multi_thread...");

    let mut stream = reqwest::get("http://localhost:3000")
        .await
        .unwrap()
        .bytes_stream();

    let mut byte_count: usize = 0;
    let start = Instant::now();

    while let Some(next) = stream.next().await {
        let bytes = next.unwrap();
        byte_count += bytes.len();
        black_box(bytes);
    }

    // server will return 16GiB of ones
    assert_eq!(byte_count, 16 * 1024 * 1024 * 1024);

    println!(
        "reqwest_multi_thread ran at an average of {} MiB/s",
        (byte_count as f64) / 1024. / 1024. / start.elapsed().as_secs() as f64
    );
}
