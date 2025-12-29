use anyhow::Result;
use clap::Parser;
use log::info;
use zurl::{InputType, classify_input};

#[derive(Parser)]
struct Cli {
    address: String,
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    if args.address.is_empty() {
        anyhow::bail!("provided address must be a non-empty string");
    }

    let parsed = classify_input(&args.address);
    match parsed {
        InputType::FullUrl(url) => {
            info!("Parsed FullUrl {:?}, opening directly", &url);
        }
        InputType::FuzzyPattern(_segments) => {
            anyhow::bail!("Opening links from a fuzzy pattern is not implemented yet!")
        }
    }
    Ok(())
}
