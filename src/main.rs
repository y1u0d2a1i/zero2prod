use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let server = run(listener).expect("Failed to start server");
    tokio::spawn(server);
    Ok(())
}
