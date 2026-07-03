/**
 * mpsc - multiple producer, single consumer
 * Here, Multiple senders and single consumers
 *
 * Thread will wait until all senders removed from the thread
 */
use tokio::sync::mpsc;

pub async fn run_channels() {
    // create a channel of type String with buffer size= 10
    // tx is sender, rx is receiver
    let (tx, mut rx) = mpsc::channel::<String>(10);

    // clone it 2 senders
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // crrate new async task - A and move tx1 ownership
    tokio::spawn(async move {
        // send message to the channel
        tx1.send("Hello This is From the Task A".to_string())
            .await
            .expect("Sent");
    });

    // crrate new async task - B and move tx2 owneship
    tokio::spawn(async move {
        // send message to the channel
        tx2.send("Hello This is From the Task B".to_string())
            .await
            .expect("Sent");
    });

    // when all senders removed then main thread exits
    drop(tx);

    // while the rx not empty just check if there value, if yes then print it
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }

    println!("Channel Close");
}
