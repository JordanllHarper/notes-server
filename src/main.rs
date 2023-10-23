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

    let ip = "127.0.0.1:2121";
    // Listen...

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    println!("Server listening on ftp://{}", ip);
    runtime.block_on(server.listen(ip))?;
    Ok(())
}
