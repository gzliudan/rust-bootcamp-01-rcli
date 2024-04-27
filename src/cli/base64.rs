use super::is_file_exist;
use crate::CmdExector;
use anyhow::Ok;
use clap::Parser;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Parser)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// The base64 string to encode
    #[arg(short, long, value_parser = is_file_exist, default_value = "-", help = "The file to encode")]
    pub input: String,

    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard", help = "The format of base64")]
    pub format: Base64Format,
}

impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encoded = crate::process_encode(&self.input, self.format)?;
        println!("{encoded}");
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// The base64 string to decode
    #[arg(short, long, value_parser = is_file_exist, default_value = "-", help = "The file to decode")]
    pub input: String,

    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard", help = "The format of base64")]
    pub format: Base64Format,
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decoded = crate::process_decode(&self.input, self.format)?;
        println!("{decoded}");
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

impl CmdExector for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Self::Encode(opts) => opts.execute().await,
            Self::Decode(opts) => opts.execute().await,
        }
    }
}
