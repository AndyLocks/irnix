use std::path::PathBuf;
use clap::Subcommand;
use clap_complete::Shell;

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
    },
    #[command(about = "Generate auto completion", help_expected = true)]
    Completions {
        #[arg(help = "Your shell name (zsh, bash, fish, elvish, powershell)")]
        shell: Shell,
    },
}