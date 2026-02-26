use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub patch: String,

    #[arg(short, long, default_value_t = true)]
    pub dry_run: bool,

    #[arg(short, long, default_value_t = true)]
    pub test: bool,

    #[arg(short, long, default_value = ".")]
    pub kernel_root: PathBuf,
}
