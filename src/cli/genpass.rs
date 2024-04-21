use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub uppercase: bool,

    #[arg(long, default_value_t = false)]
    pub lowercase: bool,

    #[arg(long, default_value_t = false)]
    pub number: bool,

    #[arg(long, default_value_t = false)]
    pub symbol: bool,
}
