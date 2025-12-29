use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use confy;
use log::debug;
use open;
use serde::{Deserialize, Serialize};
use zurl::{InputType, classify_input};

#[derive(Debug, Default, Serialize, Deserialize)]
struct ZurlConfig {
    preferred_browser: Option<String>,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand)]
enum Command {
    Open {
        address: String,
    },
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    Set { key: String, value: String },
    Get { key: String },
    Path,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    let cfg: ZurlConfig = confy::load("zurl", None).context("Failed to load configuration")?;

    match args.command {
        Command::Open { address } => {
            if address.is_empty() {
                anyhow::bail!("provided address must be a non-empty string");
            }

            let parsed = classify_input(&address);
            match parsed {
                InputType::FullUrl(url) => {
                    debug!("Parsed FullUrl {:?}", &url);
                    match cfg.preferred_browser {
                        Some(browser) => {
                            debug!("Opening link with {:?}", &browser);
                            open::with(url.as_str(), browser)?
                        }
                        None => open::that(url.as_str())?,
                    }
                }
                InputType::FuzzyPattern(_segments) => {
                    anyhow::bail!("Opening links from a fuzzy pattern is not implemented yet!")
                }
            }
        }
        Command::Config { action } => {
            debug!("Received config action: {:?}", &action);
            anyhow::bail!("Config command is not implemented yet!")
        }
    }
    Ok(())
}
