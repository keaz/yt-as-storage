use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    /// Path of the file
    #[arg(short, long)]
    pub file_path: String,

    /// Output folder
    #[arg(short, long)]
    pub output_path: String,

    /// Mode
    #[arg(short, long, value_enum)]
    pub mode: Mode,
}

#[derive(Parser, Debug, ValueEnum)]
pub enum Mode {
    Encode,
    Decode,
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        match self {
            Self::Encode => Self::Encode,
            Self::Decode => Self::Decode,
        }
    }
}
