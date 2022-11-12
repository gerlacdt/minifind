use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// directory to start search recursively
    pub directory: String,

    #[arg(short, long)]
    /// regular expression to filter filenames
    pub pattern: Option<String>,

    #[arg(short, long)]
    /// filetype: d or f for directory or normal file
    pub filetype: Option<String>,
}
