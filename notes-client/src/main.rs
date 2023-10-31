use std::{arch, fs};

use clap::{Parser, Subcommand};
use ftp::FtpStream;
use md_interactor::{list_current_dir, make_md_notes};
use serde::Deserialize;

mod md_interactor;
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
fn read_from_json<T>(file_name: &str, error_message: &str) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let file_contents = fs::read_to_string(file_name).expect(error_message);
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
        #[arg(long)]
        specifier: Option<String>,
    }, //notes_ pull OR notes_ pull * (pull all) OR notes_ pull [filename]
    /// Pull files from the server. Use with * to overwrite all or specify a name to overwrite.
    Push {
        /// The file (or files) to push to the server. E.g. * or my_file.md
        #[arg(long)]
        specifier: Option<String>,
    }, //notes_ push OR notes_ push * (push all and overwrite file) OR notes_ push [filename] (overwrite a specific filename)
}

fn main() -> anyhow::Result<()> {
    let creds = read_from_json::<Credentials>(
        "credentials.json",
        "Make sure to create a valid credentials.json file.\nAttributes should be:\n\nusername: [a valid username]\npassword: [a valid password]",
    )?;
    let configuration = read_from_json::<Configuration>("configuration.json", "Looks like you haven't got a configuration.json file. Create one in your path:\nAttributes should be: \n\n ip_addr: [your local ip]\nfilename: [your notes filename]")?;

    let args = Cli::parse();
    let ftp_server_ip = configuration.ip_addr;
    let mut ftp_stream = FtpStream::connect(&ftp_server_ip).expect(&format!(
        "Check your server IP is running on {}",
        ftp_server_ip
    ));

    let _ = ftp_stream
        .login(&creds.username, &creds.password)
        .expect("Make sure login credentials are correct.");

    let filename = configuration.filename;
    // let _ = make_md_notes(&mut ftp_stream, &filename)?;

    Ok(())
}
