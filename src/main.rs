// rcli csv --file data.csv --output data.json --header -d ','

use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.file, &opts.output)?;
        }
    }

    Ok(())
}
