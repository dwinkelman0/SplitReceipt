mod cli;
mod receipt;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    args.main()
}
