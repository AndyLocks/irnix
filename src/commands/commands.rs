use std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Execute the method",
        alias = "e",
        help_expected = true
    )]
    Execute {
        #[arg(help = "Path to method. For example `object.method`")]
        method: String,

        #[arg(trailing_var_arg = true)]
        args: Vec<String>,

        #[arg(short, long, help = "Directory path")]
        namespace: Option<PathBuf>,
    }
}