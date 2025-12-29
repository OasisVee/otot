use clap::Parser;

#[derive(Parser)]
struct Cli {
    address: String,
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    println!("Hello, world!");
}
