use unftp_sbe_fs::ServerExt;

#[tokio::main]
pub async fn main() {
    // Startup server
    let ftp_home = std::env::temp_dir();
    let server = libunftp::Server::with_fs(ftp_home)
        .greeting("Welcome to my ftp server.")
        .passive_ports(50000..65535);



    let ip = "127.0.0.1:2121";
    // Listen...
    println!("Server listening on ftp://{}", ip);
    let _= server.listen(ip).await;
}
