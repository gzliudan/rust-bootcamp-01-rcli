use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// rcli csv -i input.csv -o output.csv --header -d ','

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opt) => {
            let output = if let Some(output) = csv_opt.output {
                output.clone()
            } else {
                format!("output.{}", csv_opt.format)
            };

            process_csv(&csv_opt.input, output, csv_opt.format)?;
        }
    }

    Ok(())
}
