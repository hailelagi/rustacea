use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<Option<Bytes>>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // Establish a connection to the server
    let client = client::connect("127.0.0.1:6379").await.unwrap();

    // Create a new channel with a capacity of at most 32.
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    // // Spawn two tasks, one gets a key, the other sets a key
    // let t1 = tokio::spawn(async move {
    //     let cmd = Command::Get {
    //         key: "foo".to_string(),
    //     };

    //     tx.send(cmd).await.unwrap();
    // });

    let t1 = tokio::spawn(async move {
        let (tx, rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "test".to_string(),
            resp: tx,
        };
    });

    let t2 = tokio::spawn(async move {
        let (tx, rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: tx,
        };

        tx2.send(cmd).await.unwrap();
        
        // Await the response
        let res = rx.await;
        println!("GOT = {:?}", res);
    });

    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::Get { key, resp } => {
                let res = client.get(&key).await;
                // Ignore errors
                let _ = resp.send(res);
            }
            Command::Set { key, val, resp } => {
                let res = client.set(&key, val).await;
                // Ignore errors
                let _ = resp.send(res);
            }
        }
    }
}
