mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};

#[derive(Debug, Parser)]
#[clap(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand, about = "base64 encode/decode")]
    Base64(Base64SubCommand),

    #[command(subcommand, about = "text sign/verify")]
    Text(TextSubCommand),

    #[command(subcommand, about = "http server")]
    Http(HttpSubCommand),
}

fn is_file_exist(path: &str) -> Result<String, String> {
    // if input is "-" or file exists
    if path == "-" || Path::new(path).exists() {
        Ok(path.into())
    } else {
        Err(format!("File '{path}' does not exist"))
    }
}

fn is_dir_exist(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_file_exist() {
        assert_eq!(
            is_file_exist("-"),
            Ok("-".into()),
            "is_file_exist should return the input if it is '-'"
        );

        assert_eq!(
            is_file_exist("*"),
            Err("File '*' does not exist".into()),
            "is_file_exist should return the input if it is '*'"
        );

        assert_eq!(
            is_file_exist("Cargo.toml"),
            Ok("Cargo.toml".into()),
            "is_file_exist should return the input if the file 'Cargo.toml' exists"
        );

        assert_eq!(
            is_file_exist("not_exist.txt"),
            Err("File 'not_exist.txt' does not exist".into()),
            "is_file_exist should return an error if the file does not exist"
        )
    }
}
