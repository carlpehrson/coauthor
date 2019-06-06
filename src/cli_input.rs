use crate::coauthor::Coauthor;
use std::io::{stdin, stdout, Write};

pub fn request_new_coauthor() -> Coauthor {
    let username = ask_for_value("Username");
    let name = ask_for_value("Full name");
    let email = ask_for_value("email");

    Coauthor {
        username,
        name,
        email,
    }
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
