mod cli_input;
mod coauthor;
mod coauthors_file;
mod input_command;

use std::env;
use coauthor::Coauthor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = input_command::parse_command(args);

    match command {
        input_command::InputCommand::Add => {
            let coauthor = cli_input::request_new_coauthor();
            coauthors_file::store_coauthor(coauthor.clone());
        }
        input_command::InputCommand::List => {
            let coauthors = coauthors_file::read_coauthors();
            for coauthor in coauthors {
                print_coauthor(coauthor);
            }
        },
        input_command::InputCommand::Help => print_help_section(),
        input_command::InputCommand::Unknown(action) => {
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
      add             Starts a prompt to add an coauthor.
      list            Lists all stored coauthors.

      help            Show this help section.
    "#
    );
}
