use ftp::FtpStream;

pub enum FilePullStrategy {
    All,
    SpecificFile { filename: String },
}

pub fn match_arg(opt_arg: Option<String>) -> FilePullStrategy {
    // NOTE: Could we have it so that we can exclude files to pull?
    //
    match opt_arg {
        Some(v) => match_some_value(&v),
        None => FilePullStrategy::All,
    }
}

fn match_some_value(value: &str) -> FilePullStrategy {
    match value {
        "*" => FilePullStrategy::All,
        "." => FilePullStrategy::All,
        " " => FilePullStrategy::All,
        _ => FilePullStrategy::SpecificFile {
            filename: value.to_string(),
        },
    }
}
pub struct MarkdownFile {
    pub file_name: String,
    pub contents: Vec<u8>,
}

pub fn pull_all_files(
    ftp_stream: &mut FtpStream,
    file_name: &str,
) -> anyhow::Result<Vec<MarkdownFile>> {
    todo!()
}
pub fn pull_specific_file(
    ftp_stream: &mut FtpStream,
    filename: &str,
) -> anyhow::Result<MarkdownFile> {
    todo!()
}
