use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify, Base64SubCommand,
    HttpSubCommand, Opts, SubCommand, TextSubCommand,
};
use std::fs;
use zxcvbn::zxcvbn;

// rcli csv -i input.csv -o output.csv --header -d ','

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

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

        SubCommand::GenPass(opt) => {
            let password = process_genpass(opt.length, opt.uppercase, opt.number, opt.symbol)?;
            println!("{password}");
            // output password strength in stderr
            let estimate = zxcvbn(&password, &[])?;
            eprintln!("Password strength: {}", estimate.score());
        }

        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opt) => {
                let encoded = process_encode(&opt.input, opt.format)?;
                println!("{encoded}")
            }
            Base64SubCommand::Decode(opt) => {
                let decoded = process_decode(&opt.input, opt.format)?;
                println!("{decoded}")
            }
        },

        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opt) => {
                let signed = process_text_sign(&opt.input, &opt.key, opt.format)?;
                println!("{signed}")
            }
            TextSubCommand::Verify(opt) => {
                let is_ok = process_text_verify(&opt.input, &opt.key, opt.format, &opt.sig)?;
                println!("{is_ok}")
            }
            TextSubCommand::Generate(opt) => {
                let key = process_text_generate(opt.format)?;
                match opt.format {
                    rcli::TextSignFormat::Blake3 => {
                        let name = opt.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        let name = &opt.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },

        SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opt) => {
                process_http_serve(opt.dir, opt.port).await?;
            }
        },
    }

    Ok(())
}
