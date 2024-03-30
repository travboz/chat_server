use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // this is our server address - clients connect to this
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        // we accept a new connection at `socket`
        let (mut socket, _) = listener.accept().await.unwrap();

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
                // read_line doesn't clear out the buffer, it just appends onto `line`.
                let bytes_read = reader.read_line(&mut line).await.unwrap();

                // no bytes have been sent, so just quit
                if bytes_read == 0 {
                    break;
                }

                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear(); // clearing out or removing all the data in `line`
            }
        });
    }
}
