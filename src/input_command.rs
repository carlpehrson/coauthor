#[derive(Debug, PartialEq, Eq)]
pub enum InputCommand {
    Add,
    Help,
    Unknown(String),
}

pub fn parse_command(arguments: Vec<String>) -> InputCommand {
    match arguments[1].as_ref() {
        "a" | "add" => InputCommand::Add,
        "h" | "help" => InputCommand::Help,
        anything => InputCommand::Unknown(anything.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn test_add_command_parsing() {
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "a"]),
            InputCommand::Add
        );
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "add"]),
            InputCommand::Add
        );
    }

    #[test]
    fn test_help_command_parsing() {
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "h"]),
            InputCommand::Help
        );
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "help"]),
            InputCommand::Help
        );
    }

    #[test]
    fn test_unkown_command_parsing() {
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "anything"]),
            InputCommand::Unknown(String::from("anything"))
        );
    }
}
