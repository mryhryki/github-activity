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
    issueComments: IssueCommentNode,
}

#[derive(Deserialize, Debug)]
struct IssueCommentNode {
    nodes: Vec<GitHubIssueCommentNode>,
    pageInfo: PageInfo,
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

#[derive(Deserialize, Debug)]
struct PageInfo {
    pub endCursor: Option<String>,
}

#[derive(Serialize, Debug)]
struct Variables {
    After: Option<String>,
}

pub async fn get_issue_comments() -> Result<Vec<GitHubIssueCommentNode>, Box<dyn std::error::Error>>
{
    print!("Collect issue comments");
    let mut issue_comments: Vec<GitHubIssueCommentNode> = vec![];
    let mut end_cursor: Option<String> = None;

    loop {
        print!(".");
        io::stdout().flush().unwrap();
        let query = String::from("
          query ($After: String) {
            viewer {
              issueComments(first: 100, orderBy: {field: UPDATED_AT, direction: DESC}, after: $After) {
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
        ");
        let variables = Variables {
            After: end_cursor,
        };

        let response = request_github_graphql_api(query, variables).await?;
        if response.status() != 200 {
            return Err(Box::new(Error::new(ErrorKind::Other, "Failed get_issue_comments")));
        }

        let json = response.json::<ResponseRoot>().await?;
        for issue_comment_node in json.data.viewer.issueComments.nodes {
            issue_comments.push(issue_comment_node)
        }
        end_cursor = json.data.viewer.issueComments.pageInfo.endCursor;
        if end_cursor.is_none() {
            break;
        }
    }

    println!();
    Ok(issue_comments)
}
