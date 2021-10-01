#![allow(non_snake_case)]
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug)]
pub enum OwnerType {
    User,
    Organization,
}

impl fmt::Display for OwnerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OwnerType::User => f.write_str("User"),
            OwnerType::Organization => f.write_str("Organization"),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct RepoLogin {
    pub login: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GitHubRepository {
    pub name: String,
    pub owner: RepoLogin,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GitHubIssueNode {
    pub number: i32,
    pub url: String,
    pub title: String,
    pub createdAt: String,
    pub updatedAt: String,
}
