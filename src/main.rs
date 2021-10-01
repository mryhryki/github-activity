use crate::github::github_issue::get_issue_activity;

mod github;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let issue_activity = get_issue_activity().await?;
    println!("{:#?}", issue_activity);

    Ok(())
}
