use clap::Parser;
use minifind::clap::Args;
use minifind::{find, Options};
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let pattern = args.pattern.map(|pattern| Regex::new(&pattern).unwrap());
    let opts = Options {
        directory: args.directory,
        pattern,
        filetype: args.filetype,
    };
    find(opts)?;
    Ok(())
}
