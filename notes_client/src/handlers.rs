use anyhow::Ok;
use ftp::FtpStream;

use crate::handler_utils::{self, match_arg, pull_all_files, MarkdownFile};

pub enum FileCollection {
    Single(MarkdownFile),
    Multiple(Vec<MarkdownFile>),
}

pub fn pull_docs(
    ftp_stream: &mut FtpStream,
    option: Option<String>,
    root_filename: String,
) -> anyhow::Result<FileCollection> {
    let strategy = match_arg(option);
    match strategy {
        crate::handler_utils::FilePullStrategy::All => {
            let file_collection = pull_all_files(ftp_stream, &root_filename)?;
            return Ok(FileCollection::Multiple(file_collection));
        }
        crate::handler_utils::FilePullStrategy::SpecificFile { filename } => {
            let file = handler_utils::pull_specific_file(ftp_stream, &filename)?;
            return Ok(FileCollection::Single(file));
        }
    }
}

pub fn push_docs(option: Option<String>, root_filename: String) -> anyhow::Result<()> {
    println!("Push docs ran");
    Ok(())
}
