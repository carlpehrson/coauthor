#[derive(Debug, PartialEq, Eq)]
pub enum InputCommand {
    Add,
    List,
    Remove(String),
    Set(Vec<String>),
    Clear,
    Help,
    Unknown(String),
}

pub fn parse_command(arguments: Vec<String>) -> Result<InputCommand, &'static str> {
    if arguments.get(1).is_none() {
        return Ok(InputCommand::Help);
    }

    match arguments[1].as_ref() {
        "a" | "add" => Ok(InputCommand::Add),

        "l" | "list" => Ok(InputCommand::List),

        "r" | "remove" => {
            if arguments.len() < 3 {
                return Err("No username defined");
            }

            return Ok(InputCommand::Remove(arguments[2].to_string()));
        }

        "s" | "set" => {
            if arguments.len() < 3 {
                return Err("No coauthor(s) defined");
            }

            return Ok(InputCommand::Set(arguments[2..].to_vec()));
        }

        "c" | "clear" => {
            return Ok(InputCommand::Clear);
        }

        "h" | "help" => Ok(InputCommand::Help),

        anything => Ok(InputCommand::Unknown(anything.to_string())),
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
            Ok(InputCommand::Add)
        );
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "add"]),
            Ok(InputCommand::Add)
        );
    }

    #[test]
    fn test_command_parsing_with_no_input() {
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor"]),
            Ok(InputCommand::Help)
        );
    }

    #[test]
    fn test_remove_command_parsing() {
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "r", "username"]),
            Ok(InputCommand::Remove(String::from("username")))
        );
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "remove", "username"]),
            Ok(InputCommand::Remove(String::from("username")))
        );

        // When third argument is missing
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "remove"]),
            Err("No username defined")
        );
    }

    #[test]
    fn test_list_command_parsing() {
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "l"]),
            Ok(InputCommand::List)
        );
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "list"]),
            Ok(InputCommand::List)
        );
    }

    #[test]
    fn test_set_command_parsing() {
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "s", "username"]),
            Ok(InputCommand::Set(vec_of_strings!["username"]))
        );
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "set", "username"]),
            Ok(InputCommand::Set(vec_of_strings!["username"]))
        );

        // With multiple coauthors
        assert_eq!(
            parse_command(vec_of_strings![
                "bin/coauthor",
                "set",
                "username1",
                "username2"
            ]),
            Ok(InputCommand::Set(vec_of_strings!["username1", "username2"]))
        );

        // When third argument is missing
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "set"]),
            Err("No coauthor(s) defined")
        );
    }

    #[test]
    fn test_help_command_parsing() {
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "h"]),
            Ok(InputCommand::Help)
        );
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "help"]),
            Ok(InputCommand::Help)
        );
    }

    #[test]
    fn test_unkown_command_parsing() {
        assert_eq!(
            parse_command(vec_of_strings!["bin/coauthor", "anything"]),
            Ok(InputCommand::Unknown(String::from("anything")))
        );
    }
}
