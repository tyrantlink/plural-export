use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Tupper {
    pub avatar: Option<String>,
    pub avatar_url: Option<String>,
    pub banner: Option<String>,
    pub birthday: Option<String>,
    pub brackets: [String; 2],
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub group_id: u64,
    pub id: u64,
    pub last_used: DateTime<Utc>,
    pub name: String,
    pub nick: Option<String>,
    pub posts: u64,
    pub show_brackets: bool,
    pub tag: Option<String>
}
