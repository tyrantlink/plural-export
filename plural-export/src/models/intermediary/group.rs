use chrono::{DateTime, Utc};

use super::Image;

#[derive(Debug)]
pub struct Group {
    pub avatar:      Option<Image>,
    pub created:     DateTime<Utc>,
    pub description: Option<String>,
    pub id:          u64,
    pub name:        String,
    pub tag:         Option<String>
}
