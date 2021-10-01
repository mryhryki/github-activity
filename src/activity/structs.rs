use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Activity {
    pub id: String,
    pub title: String,
    pub url: String,
    pub datetime: String,
}
