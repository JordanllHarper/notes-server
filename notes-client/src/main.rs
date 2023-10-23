use std::{fmt::Display, fs};

use anyhow::Result;
use ftp::{FtpError, FtpStream};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

fn get_credentials() -> anyhow::Result<Credentials> {
    let file_contents = fs::read_to_string("credentials.json")
        .expect("Put credentials in a provided credentials.json file.");
    let creds: Credentials = serde_json::from_str(&file_contents.to_string())?;
    Ok(creds)
}

fn get_current_dir(ftp_stream: &mut FtpStream) -> anyhow::Result<String, FtpError> {
    match ftp_stream.pwd() {
        Ok(v) => Result::Ok(v),
        Err(e) => Err(e),
    }
}
fn list_current_dir(ftp_stream: &mut FtpStream) -> anyhow::Result<Vec<String>, FtpError> {
    match ftp_stream.list(None) {
        Ok(v) => Result::Ok(v),
        Err(e) => Err(e),
    }
}
fn cd_md_dir(ftp_stream: &mut FtpStream, md_path_in_ftp: &str) -> anyhow::Result<(), FtpError> {
    match ftp_stream.cwd(md_path_in_ftp) {
        Ok(v) => Result::Ok(v),
        Err(e) => Err(e),
    }
}

fn make_md_notes(ftp_stream: &mut FtpStream, filename: &str) -> anyhow::Result<(), FtpError> {
    match ftp_stream.mkdir(&format!("{}{}", "./", filename)) {
        Ok(v) => Result::Ok(v),
        Err(e) => Err(e),
    }
}

fn store_markdown_docs() {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let ftp_server_ip = "127.0.0.1:2121";
    let creds = get_credentials()?;
    let mut ftp_stream = FtpStream::connect(ftp_server_ip).expect(&format!(
        "Check your server IP is running on {}",
        ftp_server_ip
    ));

    let _ = ftp_stream
        .login(&creds.username, &creds.password)
        .expect("Make sure login credentials are correct.");

    //TODO: Check for notes dir - if exists then cd into --- else mkdir notes then cd into

    let filename = "notes";
    list_current_dir(&mut ftp_stream)?
        .iter()
        .for_each(|dir| println!("{}", dir));

    Ok(())
}
