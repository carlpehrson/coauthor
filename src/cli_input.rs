use crate::coauthor::Coauthor;
use crate::github_api;
use std::io::{stdin, stdout, Write};

pub fn request_new_coauthor() -> Coauthor {
    let username = ask_for_value("Username");
    let name = ask_for_value("Full name");
    let email = ask_for_value("Email");

    Coauthor {
        username,
        name,
        email,
    }
}

pub fn request_github_username() -> Result<Coauthor, String> {
    let username = ask_for_value("Github username");

    return github_api::get_coauthor_from_username(username.to_string());
}

fn ask_for_value(title: &'static str) -> String {
    let mut string = String::new();

    print!("{}:\n", title);

    let _ = stdout().flush();
    stdin()
        .read_line(&mut string)
        .expect("Did not enter a correct string");

    if let Some('\n') = string.chars().next_back() {
        string.pop();
    }
    if let Some('\r') = string.chars().next_back() {
        string.pop();
    }

    return string;
}
