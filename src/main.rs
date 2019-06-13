mod cli_input;
mod coauthor;
mod coauthors_file;
mod git_commit_template_file;
mod input_command;

use coauthor::Coauthor;
use exitfailure::ExitFailure;
use input_command::InputCommand;
use std::env;

fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();

    match input_command::parse_command(args) {
        Ok(command) => run_command(command),
        Err(error) => eprintln!("{}", error),
    }
    Ok(())
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
        }

        InputCommand::Remove(username) => {
            coauthors_file::remove_coauthor_by_username(username);
        }

        InputCommand::Set(usernames) => match coauthors_file::get_coauthors(usernames) {
            Ok(coauthors) => {
                git_commit_template_file::set_current_coauthors(coauthors);
            }

            Err(non_existing_usernames) => {
                println!(
                        "{} could not be found in the storage. run `coauthor list` to see which one are available",
                        non_existing_usernames.join(", ")
                    );
            }
        },

        InputCommand::Clear => {
            git_commit_template_file::set_current_coauthors(vec![]);
        }

        InputCommand::Help => print_help_section(),

        InputCommand::Unknown(action) => {
            print_unkown_command(action);
            print_help_section();
        }
    }
}

fn print_coauthor(coauthor: Coauthor) {
    println!(
        "[{}] {} <{}>",
        coauthor.username, coauthor.name, coauthor.email
    );
}

fn print_unkown_command(command: String) {
    eprintln!("`{}` is not a valid action.", command)
}

fn print_help_section() {
    println!(
        r#"
    Store coauthors and update them easily in your commit template.

    USAGE:
      add                               Starts a prompt to add an coauthor.
      list                              Lists all stored coauthors.
      remove [username]                 Removes a coauthor from the local machine.

      set [username [username ..]]      Updates the git template with predefined coauthors.
      clear                             Removes all coauthors from the commit template.

      help                              Show this help section.
    "#
    );
}
