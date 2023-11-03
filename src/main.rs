mod console;
mod error;
mod util;

// use std::{fs, io, path::PathBuf};

use std::env;

use clap::{Args, Parser, Subcommand};
use rskai::console::output;

// console
// |- init initialize env. create .<projectname> directory.
// |       if it already exists, console ask user whether he wants to clean current environment.
// |       i.e. remove .<projectname> or clean command.
// |- clean remove env directory i.e .<projectname>
// |-
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Args, Debug)]
struct ConsoleArgs {}

#[derive(Args, Debug)]
struct RunArgs {
    // #[arg(num_args(0..))]
    // direct: Option<String>,
    command: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Console(ConsoleArgs),
    Run(RunArgs),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match util::envinfo::ginit() {
        Ok(_) => (),
        Err(e) => {
            output::errorln!("failed to init application: {}", e.to_string());
        }
    }

    match cli.subcommand {
        SubCommands::Console(_args) => {
            console::console::start().await;
        }
        SubCommands::Run(args) => {
            let commands = crate::console::console::commands();
            // let args: Vec<String> = env::args().collect();
            // if args.len() < 3 {
            // }
            // let args = args.get(2..args.len() - 1).unwrap();
            // let given_command_name = args.get(0).unwrap();
            // let given_args = match args.len() {
            //     1 => "".to_string(),
            //     _ => args.get(1..args.len() - 1).unwrap().join(" "),
            // };
            if args.command.len() == 0 {
                output::errorln!("no command is given.");
                return;
            }
            let command_name = args.command.get(0).unwrap();
            let command_args = match args.command.get(1..args.command.len() - 1) {
                Some(args) => args.join(" "),
                None => "".to_string(),
            };
            for command in commands {
                if command.name == command_name.to_owned() {
                    let func = command.func;
                    func(command_args);
                    return;
                }
            }
        }
    }
}
