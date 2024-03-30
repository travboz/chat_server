use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    // this is our server address - clients connect to this
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(10);

    loop {
        // we accept a new connection at `socket`
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // spawns a new async task
        tokio::spawn(async move {
            // we split the socket into its reading and writing components
            let (reader, mut writer) = socket.split();

            // this holds our input
            let mut reader = BufReader::new(reader);
            // to hold a line
            let mut line = String::new();
            // main loop for getting input, reading it into our buffer `reader`
            // and then outputting it
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        // sending items to everything consumer (client)
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear(); // clearing out or removing all the data in `line`
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
