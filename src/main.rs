use clap::Parser;
use minifind::clap::Args;
use minifind::{find, Options};

fn main() {
    let args = Args::parse();

    let opts = Options {
        directory: args.directory,
    };

    find(opts);
}
