use crate::CmdExector;
use clap::Parser;
use zxcvbn::zxcvbn;

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

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password =
            crate::process_genpass(self.length, self.uppercase, self.number, self.symbol)?;
        println!("{password}");
        // output password strength in stderr
        let estimate = zxcvbn(&password, &[])?;
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}
