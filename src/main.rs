use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Bubble up the io::Error if we fail to bind to the address
    // or if we fail to run the server
    // Otherwise call .await() on our server
    run(listener)?.await
}
