use std::fs;

use clap::{Parser, Subcommand};
use ftp::FtpStream;
use handlers::{pull_docs, push_docs};
use serde::Deserialize;

mod ftp_utils;
mod handler_utils;
mod handlers;

enum ErrorStrings {
    CredsError,
    ConfigError,
}

impl ErrorStrings {
    pub fn match_error_message(&self) -> String {
        match &self {
            ErrorStrings::CredsError => String::from("Make sure to create a valid credentials.json file.\nAttributes should be:\n\nusername: [a valid username]\npassword: [a valid password]"),
            ErrorStrings::ConfigError =>String::from("Looks like you haven't got a configuration.json file. Create one in your path:\nAttributes should be: \n\n ip_addr: [your local ip]\nfilename: [your notes filename]"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Configuration {
    ip_addr: String,
    filename: String,
}
fn read_from_json<T>(file_name: &str) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let file_contents = fs::read_to_string(file_name)?;
    let obj: T = serde_json::from_str(&file_contents.to_string())?;
    Ok(obj)
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    specifier: Specifiers,
}

#[derive(Subcommand)]
enum Specifiers {
    /// Pull files from the server. Use with * to overwrite all or specify a name.
    Pull {
        /// The file to pull from the server. E.g. * to pull all, or my_file.md to pull a specific
        /// file.
        #[arg(short, long)]
        specifier: Option<String>,
    }, //notes_ pull OR notes_ pull * (pull all) OR notes_ pull [filename]
    /// Pull files from the server. Use with * to overwrite all or specify a name to overwrite.
    Push {
        /// The file (or files) to push to the server. E.g. * or my_file.md
        #[arg(short, long)]
        specifier: Option<String>,
    }, //notes_ push OR notes_ push * (push all and overwrite file) OR notes_ push [filename] (overwrite a specific filename)
}

fn handle_specifier(specifier: Specifiers, root_filename: String) -> anyhow::Result<()> {
    match specifier {
        Specifiers::Pull { specifier } => pull_docs(specifier, root_filename),
        Specifiers::Push { specifier } => push_docs(specifier, root_filename),
    }
}

fn main() -> anyhow::Result<()> {
    let creds = read_from_json::<Credentials>("credentials.json")
        .expect(&ErrorStrings::CredsError.match_error_message());
    let configuration = read_from_json::<Configuration>("configuration.json")
        .expect(&ErrorStrings::ConfigError.match_error_message());
    let args = Cli::parse();
    let ftp_server_ip = configuration.ip_addr;
    let mut ftp_stream = FtpStream::connect(&ftp_server_ip).expect(&format!(
        "Check your server IP is running on {}",
        ftp_server_ip
    ));

    let _ = ftp_stream
        .login(&creds.username, &creds.password)
        .expect("Make sure login credentials are correct.");

    let root_filename = configuration.filename;

    handle_specifier(args.specifier, root_filename.to_string())?;

    Ok(())
}
