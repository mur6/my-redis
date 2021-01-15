use mini_redis::client;
use tokio::sync::mpsc;
use bytes::Bytes;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}

#[tokio::main]
async fn main() {
    // Create a new channel with a capacity of at most 32.
    let (mut tx, mut rx) = mpsc::channel(32);
    let mut tx2 = tx.clone();

    // Establish a connection to the server
    //let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        tx.send("hello").await;
    });

    let t2 = tokio::spawn(async move {
        tx2.send("foo").await;
    });

    //t1.await.unwrap();
    //t2.await.unwrap();
    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
