use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// directory to start search recursively
    pub directory: String,
}
