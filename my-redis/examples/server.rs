use mini_redis::{Connection, Frame};
use std::{
    collections::HashMap,
    string::String,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("listening on http:://localhost:6379");

    let db: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        tokio::spawn(async move { do_stuff(socket, db).await });
    }
}

async fn do_stuff(socket: TcpStream, db: Arc<Mutex<HashMap<String, String>>>) {
    use mini_redis::Command::{self, Get, Set};
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let res = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(
                    cmd.key().to_string(),
                    std::str::from_utf8(cmd.value()).unwrap().to_string(),
                );

                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        conn.write_frame(&res).await.unwrap();

        println!("{:?}", db)
    }
}
