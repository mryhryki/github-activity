#![allow(non_snake_case)]

use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct RepoLogin {
    pub login: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GitHubRepository {
    pub name: String,
    pub owner: RepoLogin,
}

impl fmt::Display for GitHubRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{owner}/{repo}",
            owner = self.owner.login,
            repo = self.name
        )
    }
}
