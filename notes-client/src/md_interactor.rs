use anyhow::{Ok, Result};
use ftp::{FtpError, FtpStream};

pub fn get_current_dir(ftp_stream: &mut FtpStream) -> anyhow::Result<String, FtpError> {
    match ftp_stream.pwd() {
        std::result::Result::Ok(v) => Result::Ok(v),
        Err(e) => Err(e),
    }
}
pub fn list_current_dir(ftp_stream: &mut FtpStream) -> anyhow::Result<Vec<String>, FtpError> {
    match ftp_stream.list(None) {
        std::result::Result::Ok(v) => Result::Ok(v),
        Err(e) => Err(e),
    }
}
pub fn cd_md_dir(ftp_stream: &mut FtpStream, md_path_in_ftp: &str) -> anyhow::Result<(), FtpError> {
    match ftp_stream.cwd(md_path_in_ftp) {
        std::result::Result::Ok(v) => Result::Ok(v),
        Err(e) => Err(e),
    }
}

pub fn make_md_notes(ftp_stream: &mut FtpStream, filename: &str) -> anyhow::Result<(), FtpError> {
    match ftp_stream.mkdir(&format!("{}{}", "./", filename)) {
        std::result::Result::Ok(v) => Result::Ok(v),
        Err(e) => Err(e),
    }
}

pub fn pull_docs(option: Option<String>, root_filename: String) -> anyhow::Result<()> {
    println!("Pull docs ran");

    Ok(())
}

pub fn push_docs(option: Option<String>, root_filename: String) -> anyhow::Result<()> {
    println!("Push docs ran");
    Ok(())
}
