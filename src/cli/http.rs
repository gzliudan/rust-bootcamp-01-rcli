use super::is_dir_exist;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "serve a directory over http")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = is_dir_exist, default_value = ".")]
    pub dir: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
