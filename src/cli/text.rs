use super::{is_dir_exist, is_file_exist};
use crate::CmdExector;
use anyhow::Ok;
use clap::Parser;
use std::{fmt, path::PathBuf, str::FromStr};
use tokio::fs;

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    /// The message to sign
    #[arg(short, long, value_parser = is_file_exist, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = is_file_exist)]
    pub key: String,

    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = crate::process_text_sign(&self.input, &self.key, self.format)?;
        println!("{signed}");
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    /// The message to verify
    #[arg(short, long, value_parser = is_file_exist, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = is_file_exist)]
    pub key: String,

    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long)]
    pub sig: String,
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let is_ok = crate::process_text_verify(&self.input, &self.key, self.format, &self.sig)?;
        println!("{is_ok}");
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = is_dir_exist)]
    pub output: PathBuf,
}

impl CmdExector for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_text_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &key[0]).await?;
                fs::write(name.join("ed25519.pk"), &key[1]).await?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with private/shared key")]
    Sign(TextSignOpts),

    #[command(name = "verify", about = "Verify a signed message")]
    Verify(TextVerifyOpts),

    #[command(name = "generate", about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
}

impl CmdExector for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await,
            TextSubCommand::Verify(opts) => opts.execute().await,
            TextSubCommand::Generate(opts) => opts.execute().await,
        }
    }
}
