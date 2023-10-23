use std::sync::Arc;

use unftp_auth_jsonfile::JsonFileAuthenticator;
use unftp_sbe_fs::ServerExt;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Set up authentication
    let authenticator = JsonFileAuthenticator::from_file(String::from("credentials.json"))?;
    // Startup server
    let server = libunftp::Server::with_fs(std::env::temp_dir())
        .authenticator(Arc::new(authenticator))
        .greeting("Welcome to my ftp server.")
        .passive_ports(50000..65535);

    let ip = "127.0.0.1:2121";
    // Listen...
    println!("Server listening on ftp://{}", ip);
    let _ = server.listen(ip).await;
    Ok(())
}
