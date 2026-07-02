/**
 * Why Tokio ?
 * Rust know that async, await, Future ; but doesn't provide runtime
 * TOkio provide runtime
 *
 * TOkio is the Runtime Engine for the ['async await']
 */
mod tokio_spawn;

use tokio::time::{Duration, sleep};
// use tokio_spawn::tokio_spawn_fn;

// async fn hello() {
//     println!("Start");

//     sleep(Duration::from_secs(3)).await;

//     println!("End");
// }

// #[tokio::main]
// async fn main() {

//     // @type-1 (sleep)
//     hello().await;

//     // @type-2 (spawn)
//     tokio_spawn_fn().await;
// }

async fn download() {
    println!("Download S");
    sleep(Duration::from_secs(2)).await;
    println!("Download E");
}

async fn upload() {
    println!("Upload S");
    sleep(Duration::from_secs(5)).await;
    println!("Upload E");
}

#[tokio::main]
async fn main() {
    // Type 1:
    // download completes first, then upload starts.
    // upload() cannot even start until download() finishes.
    download().await;
    upload().await;

    // Type 2:
    // Create two separate Tokio tasks.
    // Runtime can schedule them concurrently.
    // main() continues immediately after spawning them.
    let d: tokio::task::JoinHandle<()> = tokio::spawn(download());
    let u: tokio::task::JoinHandle<()> = tokio::spawn(upload());

    // Wait for both spawned tasks to finish.
    // Without these awaits, main() may finish early,
    // the Tokio runtime shuts down,
    // and the spawned tasks are cancelled.
    d.await.unwrap();
    u.await.unwrap();
}
