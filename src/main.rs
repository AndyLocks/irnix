use crate::commands::commands::Commands;
use crate::config::unwrap_namespace;
use crate::handlers::{Handle, MainHandler};
use crate::method::Method;
use clap::Parser;
use std::io::stdin;
use std::process::exit;
use clap::CommandFactory;

mod commands;
mod config;
mod contract;
mod contract_tokens;
mod handlers;
mod method;

#[derive(Parser)]
#[command(
    name = "string-pool",
    version,
    about = "String Pool",
    arg_required_else_help = false
)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

fn main() {
    match Cli::parse().command {
        Commands::Execute {
            args,
            method,
            namespace,
        } => {
            let method = Method::new(method, args, stdin(), unwrap_namespace(namespace));

            match method {
                Ok(mut method) => {
                    MainHandler::handle(&mut method);
                    method.execute();
                }
                Err(message) => {
                    eprintln!("{message}");
                    exit(1);
                }
            }
        }
        Commands::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "irnix", &mut std::io::stdout())
        }
    }
}
