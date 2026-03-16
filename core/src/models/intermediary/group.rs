use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Group {
    pub avatar_url:  Option<String>,
    pub created:     DateTime<Utc>,
    pub description: Option<String>,
    pub id:          u64,
    pub name:        String,
    pub tag:         Option<String>
}
