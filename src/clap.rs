use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Args {
    /// directory to start search recursively
    pub directory: String,

    #[arg(short, long)]
    /// regular expression to filter filenames
    pub pattern: Option<String>,

    #[arg(short, long, value_enum)]
    /// filetype to search for
    pub filetype: Option<FileType>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FileType {
    Dir,
    File,
}
