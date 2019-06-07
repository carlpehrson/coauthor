mod cli_input;
mod coauthor;
mod coauthors_file;
mod input_command;

use std::env;
use coauthor::Coauthor;
use input_command::InputCommand;

fn main() {
    let args: Vec<String> = env::args().collect();

    match input_command::parse_command(args) {
        Ok(command) => run_command(command),
        Err(error) => eprintln!("{}", error),
    }
}

fn run_command(command: InputCommand) {
    match command {
        InputCommand::Add => {
            let coauthor = cli_input::request_new_coauthor();
            coauthors_file::store_coauthor(coauthor.clone());
        }

        InputCommand::List => {
            let coauthors = coauthors_file::read_coauthors();
            for coauthor in coauthors {
                print_coauthor(coauthor);
            }
        },

        InputCommand::Remove(username) => {
            coauthors_file::remove_coauthor_by_username(username);
        }

        InputCommand::Help => print_help_section(),

        InputCommand::Unknown(action) => {
            print_unkown_command(action);
            print_help_section();
        }
    }
}

fn print_coauthor(coauthor: Coauthor) {
    println!("[{}] {} <{}>", coauthor.username, coauthor.name, coauthor.email);
}

fn print_unkown_command(command: String) {
    eprintln!("`{}` is not a valid action.", command)
}

fn print_help_section() {
    println!(
        r#"
    Store coauthors and update them easily in your commit template.

    USAGE:
      add                   Starts a prompt to add an coauthor.
      list                  Lists all stored coauthors.
      remove [username]     Removes a coauthor from the local machine.

      help                  Show this help section.
    "#
    );
}
