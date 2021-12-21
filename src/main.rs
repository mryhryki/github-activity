use crate::activity::structs::Activity;
use crate::activity::writer::write_activities;
use crate::github::github_issue::get_issues;
use crate::github::github_issue_comment::get_issue_comments;
use crate::github::github_pull_request::get_pull_requests;
use chrono::DateTime;

mod activity;
mod github;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let from = DateTime::parse_from_rfc3339("2021-12-21T00:00:00+09:00").unwrap();
    let to = DateTime::parse_from_rfc3339("2021-12-21T23:59:59+09:00").unwrap();
    let filter = |iso8601: &String| -> bool {
        let time = DateTime::parse_from_rfc3339(iso8601).unwrap();
        from <= time && time < to
    };

    let mut activities: Vec<Activity> = vec![];

    for issue in get_issues().await? {
        if filter(&issue.updatedAt) {
            activities.push(Activity {
                repo: issue.repository.to_string(),
                title: format!("#{} Issue: {}", issue.number, issue.title),
                url: issue.url,
                datetime: issue.updatedAt,
            });
        }
    }

    for comment in get_issue_comments().await? {
        let mut title = comment
            .bodyText
            .as_str()
            .chars()
            .take(100)
            .collect::<String>();
        if comment.bodyText.len() > 100 {
            title = format!("{}...", title)
        }
        if filter(&comment.createdAt) {
            activities.push(Activity {
                repo: comment.repository.to_string(),
                title: format!("#{} IssueComment: {}", comment.issue.number, title),
                url: comment.url,
                datetime: comment.createdAt,
            });
        }
    }

    for pr in get_pull_requests().await? {
        if filter(&pr.updatedAt) {
            activities.push(Activity {
                repo: pr.repository.to_string(),
                title: format!("#{} PR: {}", pr.number, pr.title),
                url: pr.url,
                datetime: pr.updatedAt,
            });
        }
    }

    write_activities(activities);
    Ok(())
}
