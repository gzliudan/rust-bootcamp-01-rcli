use super::is_dir_exist;
use crate::CmdExector;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = is_dir_exist, default_value = ".")]
    pub dir: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExector for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_http_serve(self.dir, self.port).await
    }
}

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "serve a directory over http")]
    Serve(HttpServeOpts),
}

impl CmdExector for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => opts.execute().await,
        }
    }
}
