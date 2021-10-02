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
    issues: Issue,
}

#[derive(Deserialize, Debug)]
struct Issue {
    nodes: Vec<IssueNode>,
    pageInfo: PageInfo,
}

#[derive(Deserialize, Clone, Debug)]
pub struct IssueNode {
    pub number: i32,
    pub url: String,
    pub title: String,
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

pub async fn get_issues() -> Result<Vec<IssueNode>, Box<dyn std::error::Error>> {
    print!("Collect issues");
    let mut issues: Vec<IssueNode> = vec![];
    let mut end_cursor: Option<String> = None;

    loop {
        print!(".");
        io::stdout().flush().unwrap();
        let query = String::from("
          query ($After: String) {
            viewer {
              issues(first: 100, orderBy: {field: UPDATED_AT, direction: DESC}, after: $After) {
                nodes {
                  number
                  url
                  title
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
            return Err(Box::new(Error::new(ErrorKind::Other, "Failed get_issues")));
        }

        let json = response.json::<ResponseRoot>().await?;
        for issue_node in json.data.viewer.issues.nodes {
            issues.push(issue_node)
        }
        end_cursor = json.data.viewer.issues.pageInfo.endCursor;
        if end_cursor.is_none() {
            break;
        }
    }

    println!();
    Ok(issues)
}
