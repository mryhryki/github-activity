#[derive(Clone, Debug)]
pub struct Activity {
    pub id: String,
    pub repo: String, // e.g. "ORG/REPO"
    pub title: String,
    pub url: String,
    pub datetime: String,
}
