use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub num: usize,
    pub author_id: String,
    pub text: String,
}