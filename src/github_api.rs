extern crate reqwest;

use crate::coauthor::Coauthor;
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubUser {
    id: i64,
    login: String,
    name: String,
}

pub fn get_coauthor_from_username(username: String) -> Coauthor {
    let url = format!(
        "https://api.github.com/users/{username}",
        username = username
    );
    let uri = Url::parse(&url).unwrap();

    let response = reqwest::get(uri);

    let user: GithubUser = response.unwrap().json().unwrap();

    return Coauthor {
        username: user.login.clone(),
        name: user.name.clone(),
        email: format!(
            "{user_id}+{username}@users.noreply.github.com",
            user_id = user.id,
            username = user.login
        ),
    };
}
