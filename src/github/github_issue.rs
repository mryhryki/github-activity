use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

use crate::github::github_api::request_github_graphql_api;
use crate::github::structs::{GitHubIssueNode};

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
    issues: IssueNode,
}

#[derive(Deserialize, Debug)]
struct IssueNode {
    nodes: Vec<GitHubIssueNode>,
}

#[derive(Serialize, Debug)]
struct Variables {}

pub async fn get_issue_activity() -> Result<Vec<GitHubIssueNode>, Box<dyn std::error::Error>> {
    let query = String::from("
        {
          viewer {
            issues(first: 100, orderBy: {field: UPDATED_AT, direction: DESC}) {
              nodes {
                number
                url
                title
                createdAt
                updatedAt
                labels(first: 10) {
                  nodes {
                    name
                  }
                }
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
        Ok(json.data.viewer.issues.nodes)
    } else {
        Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed get_issue_activity",
        )))
    }
}
