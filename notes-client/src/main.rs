use std::{fmt::Display, fs};

use anyhow::Ok;
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

fn main() -> anyhow::Result<()> {
    let creds = get_credentials()?;

    Ok(())
}
