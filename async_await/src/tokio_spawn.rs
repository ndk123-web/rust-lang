/**
 * .await -> wait until finished download()
 * spanw -> create new task() for download() and go next immediately
 */

use tokio::time::{Duration, sleep};

async fn download() {
    println!("Download Staretd");

    // wait 3 seconds to end the download function
    sleep(Duration::from_secs(3)).await;

    println!("Download End");
}

#[allow(dead_code)]
pub async fn tokio_spawn_fn() {
    tokio::spawn(download());

    // wait 4 sec (for main process to end)
    sleep(Duration::from_secs(4)).await;

    println!("Tokio Spawn End")
}
