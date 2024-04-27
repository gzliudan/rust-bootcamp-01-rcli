use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

use crate::CmdExector;

use super::is_dir_exist;

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
#[enum_dispatch(CmdExector)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "serve a directory over http")]
    Serve(HttpServeOpts),
}
