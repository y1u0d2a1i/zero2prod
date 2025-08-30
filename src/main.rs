use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configration().expect("Failed to read configuration.");
    
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let server = run(listener).expect("Failed to start server");
    tokio::spawn(server);
    Ok(())
}
