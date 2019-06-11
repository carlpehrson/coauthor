use crate::coauthor::Coauthor;
use shellexpand::tilde;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn set_current_coauthors(coauthors: Vec<Coauthor>) {
    let coauthor_strings: Vec<String> = coauthors
        .iter()
        .map(|coauthor| to_coauthored_string(coauthor))
        .collect();

    let template_string = coauthor_template_string(coauthor_strings);

    fs::write(template_file(), &template_string).expect("Unable to write file");
}

fn template_file() -> String {
    let output = Command::new("git")
        .args(&["config", "commit.template"])
        .output()
        .expect("Failed to retrieve the configured git template");

    let mut path = String::from_utf8_lossy(&output.stdout).to_string();
    path.pop(); // Removes trailing `\n`

    if path == "" {
        let file_path_string = tilde("~/.config/git/.gitmessage").to_string();
        let file_path = Path::new(&file_path_string);
        let _ = fs::File::create(file_path);
        println!("commit template missing. Added it to {}", file_path_string);

        return file_path_string;
    }

    return path;
}

fn coauthor_template_string(coauthor_strings: Vec<String>) -> String {
    return [
        "", // The convention is that there should be two empty lines
        "", // in a row before the coauthors are defined
        "# Coauthors managed by coauthor:",
        &coauthor_strings.join("\n"),
        "# coauthor end",
    ]
    .join("\n");
}

fn to_coauthored_string(coauthor: &Coauthor) -> String {
    return format!("Co-Authored-By: {} <{}>", coauthor.name, coauthor.email);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_coauthored_string_test() {
        let coauthor = Coauthor {
            username: "johantell".to_string(),
            name: "Johan Tell".to_string(),
            email: "johan.tell@example.com".to_string(),
        };

        assert_eq!(
            to_coauthored_string(&coauthor),
            "Co-Authored-By: Johan Tell <johan.tell@example.com>"
        );
    }
}
