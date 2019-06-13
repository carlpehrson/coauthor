use crate::coauthor::Coauthor;
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct CoauthorsStorage {
    coauthors: Vec<Coauthor>,
}

fn coauthors_file() -> String {
    let file_path_string = tilde("~/.config/coauthor/coauthors.toml").to_string();
    let file_path = Path::new(&file_path_string);

    if !Path::new(&file_path).exists() {
        let _ = fs::File::create(file_path);
    }

    return file_path_string;
}

pub fn store_coauthor(coauthor: Coauthor) {
    let mut coauthors = read_coauthors();
    coauthors.push(coauthor);

    write_coauthors(coauthors);
}

pub fn remove_coauthor_by_username(username: String) {
    let filtered_coauthors = read_coauthors()
        .iter()
        .filter(|coauthor| coauthor.username != username)
        .cloned()
        .collect();

    write_coauthors(filtered_coauthors);
}

pub fn get_coauthor_by_email(email: String) -> Option<Coauthor> {
    let coauthors = read_coauthors();

    return coauthors
        .iter()
        .find(|coauthor| coauthor.email == email)
        .cloned();
}

pub fn get_coauthors(coauthors: Vec<String>) -> Result<Vec<Coauthor>, Vec<String>> {
    let coauthors_structs = read_coauthors();

    let result: (Vec<Coauthor>, Vec<String>) = coauthors.iter().fold(
        (vec![], vec![]),
        |(mut matching_coauthors, mut missing_coauthors), username| {
            let matching_coauthor = coauthors_structs
                .iter()
                .find(|coauthor| coauthor.username == username.to_string())
                .cloned();

            match matching_coauthor {
                Some(coauthor) => matching_coauthors.push(coauthor),
                None => missing_coauthors.push(username.to_string()),
            }

            return (matching_coauthors, missing_coauthors);
        },
    );

    let (found_coauthors, missing_coauthors) = result;

    if missing_coauthors.len() == 0 {
        return Ok(found_coauthors);
    } else {
        return Err(missing_coauthors);
    }
}

pub fn read_coauthors() -> Vec<Coauthor> {
    let file_contents = fs::read_to_string(coauthors_file()).expect("Unable to read file");

    if file_contents == "" {
        return vec![];
    }

    let coauthors_storage: CoauthorsStorage = toml::from_str(&file_contents).unwrap();

    return coauthors_storage.coauthors;
}

fn write_coauthors(coauthors: Vec<Coauthor>) {
    let coauthor_list = CoauthorsStorage {
        coauthors: coauthors,
    };

    let toml = toml::to_string(&coauthor_list).unwrap();

    fs::write(coauthors_file(), &toml).expect("Unable to write file");
}
