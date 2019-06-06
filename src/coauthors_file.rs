use crate::coauthor::Coauthor;
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::fs;

#[derive(Serialize, Deserialize)]
struct CoauthorsStorage {
    coauthors: Vec<Coauthor>,
}

fn coauthors_file() -> String {
    return tilde("~/.config/coauthor/coauthors.toml").to_string();
}

pub fn store_coauthor(coauthor: Coauthor) {
    let mut coauthors = read_coauthors();
    coauthors.push(coauthor);
    let coauthor_list = CoauthorsStorage {
        coauthors: coauthors,
    };

    let toml = toml::to_string(&coauthor_list).unwrap();

    fs::write(coauthors_file(), &toml).expect("Unable to write file");
}

pub fn read_coauthors() -> Vec<Coauthor> {
    let file_contents = fs::read_to_string(coauthors_file()).expect("Unable to read file");
    let coauthors_storage: CoauthorsStorage = toml::from_str(&file_contents).unwrap();

    return coauthors_storage.coauthors;
}
