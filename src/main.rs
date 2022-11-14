use anyhow::Result;
use clap::Parser;
use minifind::clap::Args;
use minifind::{find, Options};
use regex::Regex;

fn main() -> Result<()> {
    match exec() {
        Ok(()) => Ok(println!("")),
        Err(e) => Ok(eprintln!("ERROR executing minifind, cause: {}", e)),
    }
}

fn exec() -> Result<()> {
    let args = Args::parse();
    let pattern = args.pattern.map(|pattern| Regex::new(&pattern));
    let pattern = match pattern {
        Some(p) => Some(p?),
        None => None,
    };

    let opts = Options {
        directory: args.directory,
        pattern,
        filetype: args.filetype,
    };
    find(opts)?;
    Ok(())
}
