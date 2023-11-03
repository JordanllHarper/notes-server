use ftp::FtpStream;

use crate::handler_utils::match_arg;

pub fn pull_docs(
    ftp_stream: FtpStream,
    option: Option<String>,
    root_filename: String,
) -> anyhow::Result<()> {
    let strategy = match_arg(option);
    match strategy {
        crate::handler_utils::FilePullStrategy::All => todo!(),
        crate::handler_utils::FilePullStrategy::SpecificFile { filename } => todo!(),
    }

    Ok(())
}

pub fn push_docs(option: Option<String>, root_filename: String) -> anyhow::Result<()> {
    println!("Push docs ran");
    Ok(())
}
