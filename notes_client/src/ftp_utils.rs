use std::io::Cursor;

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

pub fn write_contents_to_ftp(
    ftp_stream: &mut FtpStream,
    file_name_as_stored: &str,
    contents: &str,
) -> anyhow::Result<()> {
    let mut reader = Cursor::new(contents.as_bytes());
    let _result = ftp_stream.put(file_name_as_stored, &mut reader)?;
    Ok(())
}

pub fn pull_file(ftp_stream: &mut FtpStream, filename: &str) -> anyhow::Result<Vec<u8>> {
    let result = ftp_stream.simple_retr(filename)?;
    let inner = result.into_inner();
    Ok(inner)
}
