#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

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
    issueComments: IssueCommentNode,
}

#[derive(Deserialize, Debug)]
struct IssueCommentNode {
    nodes: Vec<GitHubIssueCommentNode>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GitHubIssueCommentNode {
    pub url: String,
    pub bodyText: String,
    pub createdAt: String,
    pub issue: IssueNode,
    pub repository: GitHubRepository,
}

#[derive(Deserialize, Clone, Debug)]
pub struct IssueNode {
    pub number: i32,
}

#[derive(Serialize, Debug)]
struct Variables {}

pub async fn get_issue_comments() -> Result<Vec<GitHubIssueCommentNode>, Box<dyn std::error::Error>>
{
    let query = String::from(
        "
      {
        viewer {
          issueComments(first: 100, orderBy: {field: UPDATED_AT, direction: DESC}) {
            nodes {
              url
              bodyText
              createdAt
              issue {
                number
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
    ",
    );
    let variables = Variables {};

    let response = request_github_graphql_api(query, variables).await?;
    if response.status() == 200 {
        let json = response.json::<ResponseRoot>().await?;
        Ok(json.data.viewer.issueComments.nodes)
    } else {
        Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed get_issue_activity",
        )))
    }
}
