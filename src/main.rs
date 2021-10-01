use crate::activity::structs::Activity;
use crate::github::github_issue::get_issue_activity;
use crate::github::github_issue_comment::get_issue_comments;

mod activity;
mod github;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut activities: Vec<Activity> = vec![];

    for issue in get_issue_activity().await? {
        activities.push(Activity {
            id: format!("issue:{}", issue.number),
            title: issue.title,
            url: issue.url,
            datetime: issue.updatedAt,
        });
    }

    for comment in get_issue_comments().await? {
        activities.push(Activity {
            id: format!("issue:{}:comment", comment.issue.number),
            title: format!("Commented: {}", comment.bodyText),
            url: comment.url,
            datetime: comment.createdAt,
        });
    }

    println!("{:#?}", activities);
    Ok(())
}
