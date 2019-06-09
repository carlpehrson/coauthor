use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coauthor {
    pub username: String,
    pub name: String,
    pub email: String,
}

trait GitMessageSerializable {
    fn to_coauthored_string(coauthor: Coauthor) -> String;
}

impl GitMessageSerializable for Coauthor {
    fn to_coauthored_string(coauthor: Coauthor) -> String {
       return format!("Co-Authored-By: {} <{}>", coauthor.name, coauthor.email);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_coauthored_string_test() {
        let coauthor = Coauthor {
            username: "johantell".to_string(),
            name: "Johan Tell".to_string(),
            email: "johan.tell@example.com".to_string()
        };

        assert_eq!(Coauthor::to_coauthored_string(coauthor), "Co-Authored-By: Johan Tell <johan.tell@example.com>");
    }
}
