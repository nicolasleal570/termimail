mod api;
mod errors;
mod features;

use clap::Command;
use features::generate_email::generate_email;

fn cli() -> Command {
    Command::new("termail")
        .about("Generate temporal email domains from your terminal with ease (Mailsy alternative)")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("me").about("Show details of the current account"))
        .subcommand(Command::new("g").about("Generate a new email account"))
        .subcommand(Command::new("m").about("Fetch messages from the inbox"))
        .subcommand(Command::new("d").about("Delete current account"))
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("me", _sub_matches)) => {
            println!("ME COMMAND {:?}", matches.subcommand());
            todo!();
        }

        Some(("g", _sub_matches)) => {
            let _ = generate_email().await;
        }

        Some(("m", _sub_matches)) => {
            println!("M COMMAND {:?}", matches.subcommand());

            todo!();
        }

        Some(("d", _sub_matches)) => {
            println!("D COMMAND {:?}", matches.subcommand());

            todo!();
        }

        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
