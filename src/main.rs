use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

// rcli csv -i input.csv -o output.csv --header -d ','

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opt) => {
            let output = if let Some(output) = opt.output {
                output.clone()
            } else {
                format!("output.{}", opt.format)
            };
            process_csv(&opt.input, output, opt.format)?;
        }

        SubCommand::Genpass(opt) => {
            process_genpass(opt.length, opt.uppercase, opt.number, opt.symbol)?;
        }
    }

    Ok(())
}
