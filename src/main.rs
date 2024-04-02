use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// rcli csv -i input.csv -o output.csv --header -d ','

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opt) => process_csv(&csv_opt.input, &csv_opt.output)?,
    }

    Ok(())
}
