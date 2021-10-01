use crate::activity::structs::Activity;
use crate::github::github_issue::get_issues;
use crate::github::github_issue_comment::get_issue_comments;
use crate::github::github_pull_request::get_pull_requests;

mod activity;
mod github;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut activities: Vec<Activity> = vec![];

    for issue in get_issues().await? {
        activities.push(Activity {
            id: format!("{}", issue.number),
            repo: issue.repository.to_string(),
            title: issue.title,
            url: issue.url,
            datetime: issue.updatedAt,
        });
    }

    for comment in get_issue_comments().await? {
        activities.push(Activity {
            id: format!("{}", comment.issue.number),
            repo: comment.repository.to_string(),
            title: format!("Commented: {}", comment.bodyText),
            url: comment.url,
            datetime: comment.createdAt,
        });
    }

    for pr in get_pull_requests().await? {
        activities.push(Activity {
            id: format!("{}", pr.number),
            repo: pr.repository.to_string(),
            title: pr.title,
            url: pr.url,
            datetime: pr.updatedAt,
        });
    }

    println!("{:#?}", activities);
    Ok(())
}
