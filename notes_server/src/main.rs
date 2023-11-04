use std::{error::Error, sync::Arc};

use unftp_auth_jsonfile::JsonFileAuthenticator;
use unftp_sbe_fs::ServerExt;

pub fn main() -> Result<(), Box<dyn Error>> {
    //Set up authentication
    let authenticator = JsonFileAuthenticator::from_file(String::from("credentials.json"))?;
    let ftp_home = std::env::temp_dir();
    // Startup server
    let server = libunftp::Server::with_fs(ftp_home)
        .authenticator(Arc::new(authenticator))
        .greeting("Welcome to my ftp server.")
        .passive_ports(50000..65535);

    let ip = "127.0.0.1";
    let port = "2121";
    let ip_to_host = format!("{}:{}", ip, port);
    // Listen...
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    println!();
    println!("|||||||     Server running -_-     |||||||");
    println!();
    println!("Full address: {}", &ip_to_host);
    println!("----");
    println!("IP Address: {}", ip);
    println!("----");
    println!("Port: {}", port);
    println!();
    println!("Press CTRL + C to stop the server");

    runtime.block_on(server.listen(ip_to_host))?;
    Ok(())
}
