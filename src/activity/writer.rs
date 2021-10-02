use crate::activity::structs::Activity;
use std::cmp::Ordering;

pub fn write_activities(mut activities: Vec<Activity>) {
    activities.sort_by(|a1, a2| {
        let a1id = format!("{}:{}", a1.repo, a1.title);
        let a2id = format!("{}:{}", a2.repo, a2.title);
        if a1id > a2id {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let mut repo = String::from("");
    for activity in activities {
        if repo != activity.repo {
            repo = String::from(&activity.repo);
            println!("\n# {}\n", repo);
        }
        println!(
            "- [{}]({})",
            activity.title.replace("\n", " "),
            activity.url
        )
    }
}
