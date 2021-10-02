#![allow(non_snake_case)]

use std::io;
use std::io::{Error, ErrorKind, Write};

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
    pageInfo: PageInfo,
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

#[derive(Deserialize, Debug)]
struct PageInfo {
    pub endCursor: Option<String>,
}

#[derive(Serialize, Debug)]
struct Variables {
    After: Option<String>,
}

pub async fn get_pull_requests() -> Result<Vec<PullRequestNode>, Box<dyn std::error::Error>> {
    print!("Collect pull requests");
    let mut pull_requests: Vec<PullRequestNode> = vec![];
    let mut end_cursor: Option<String> = None;

    loop {
        print!(".");
        io::stdout().flush().unwrap();
        let query = String::from("
          query ($After: String) {
            viewer {
              pullRequests(first: 100, orderBy: {field: UPDATED_AT, direction: DESC}, after: $After) {
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
        let variables = Variables {
            After: end_cursor,
        };

        let response = request_github_graphql_api(query, variables).await?;
        if response.status() != 200 {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "Failed get_pull_requests",
            )))
        }

        let json = response.json::<ResponseRoot>().await?;
        for pull_request_node in json.data.viewer.pullRequests.nodes {
            pull_requests.push(pull_request_node)
        }
        end_cursor = json.data.viewer.pullRequests.pageInfo.endCursor;
        if end_cursor.is_none() {
            break;
        }
    }

    Ok(pull_requests)
}
