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

pub fn get_coauthor_from_username(username: String) -> Result<Coauthor, String> {
    let url = format!(
        "https://api.github.com/users/{username}",
        username = username
    );
    let uri = Url::parse(&url).unwrap();

    match make_request(uri) {
        Ok(user) => {
            let coauthor = Coauthor {
                username: user.login.clone(),
                name: user.name.clone(),
                email: format!(
                    "{user_id}+{username}@users.noreply.github.com",
                    user_id = user.id,
                    username = user.login
                ),
            };

            return Ok(coauthor);
        }

        Err(error) => {
            if error.is_http() {
                return Err("Could not perform request to github".to_string());
            }

            if error.is_serialization() {
                return Err(format!(
                    "Could not find user `{username}` on github",
                    username = username
                ));
            }

            return Err("Unkown error".to_string());
        }
    }
}

fn make_request(uri: Url) -> Result<GithubUser, reqwest::Error> {
    return reqwest::get(uri)?.json();
}
