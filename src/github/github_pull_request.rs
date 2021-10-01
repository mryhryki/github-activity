#![allow(non_snake_case)]
use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

use crate::github::github_api::request_github_graphql_api;
use crate::github::structs::GitHubRepository;

#[derive(Deserialize, Debug)]
struct ResponseRoot {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    viewer: Viewer,
}

#[derive(Deserialize, Debug)]
struct Viewer {
    pullRequests: PullRequest,
}

#[derive(Deserialize, Debug)]
struct PullRequest {
    nodes: Vec<PullRequestNode>,
}

#[derive(Deserialize, Debug)]
pub struct PullRequestNode {
    pub number: i32,
    pub url: String,
    pub title: String,
    pub bodyText: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub repository: GitHubRepository,
}

#[derive(Serialize, Debug)]
struct Variables {}

pub async fn get_pull_requests() -> Result<Vec<PullRequestNode>, Box<dyn std::error::Error>> {
    let query = String::from("
      {
        viewer {
          pullRequests(first: 10, orderBy: {field: UPDATED_AT, direction: DESC}) {
            nodes {
              number
              url
              title
              bodyText
              createdAt
              updatedAt
              repository {
                owner {
                  login
                }
                name
              }
            }
            pageInfo {
              endCursor
            }
          }
        }
      }
    ");
    let variables = Variables {};

    let response = request_github_graphql_api(query, variables).await?;
    if response.status() == 200 {
        let json = response.json::<ResponseRoot>().await?;
        Ok(json.data.viewer.pullRequests.nodes)
    } else {
        Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed get_pull_requests",
        )))
    }
}
