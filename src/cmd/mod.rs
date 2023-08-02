use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    /// Path of the file
    #[arg(short, long)]
    pub file_path: String,

    /// Output folder
    #[arg(short, long)]
    pub output_path: String,
}
