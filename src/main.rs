mod api;
mod data_struct;
mod errors;
mod features;
mod utils;

use clap::Command;
use features::{
    current_account::current_account, delete_account::delete_account, fetch_emails::fetch_emails,
    generate_email::generate_email,
};

fn cli() -> Command {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    Command::new("termail")
        .about("Generate temporal email domains from your terminal with ease (Mailsy alternative)")
        .version(VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
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
            match current_account().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error getting created account: {}", e);
                    std::process::exit(1); // Exit with a non-zero status code
                }
            }
        }

        Some(("g", _sub_matches)) => {
            match generate_email().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error generating email: {}", e);
                    std::process::exit(1); // Exit with a non-zero status code
                }
            }
        }

        Some(("m", _sub_matches)) => {
            match fetch_emails().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error while getting emails: {}", e);
                    std::process::exit(1); // Exit with a non-zero status code
                }
            }
        }

        Some(("d", _sub_matches)) => {
            match delete_account().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error while deleting account: {}", e);
                    std::process::exit(1); // Exit with a non-zero status code
                }
            }
        }

        _ => unreachable!(), // All subcommands are defined, so this should not be possible
    }
}
