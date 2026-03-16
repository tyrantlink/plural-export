use chrono::{DateTime, Utc};

use super::ProxyTag;

#[derive(Debug)]
pub struct Member {
    // TODO this breaks some stuff, avatar should probably be
    // TODO an enum of String (id), Url, or Data (ampersand)
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub birthday: Option<String>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub group: Option<u64>,
    pub id: u64,
    pub last_used: Option<DateTime<Utc>>,
    pub message_count: u64,
    pub name: String,
    pub proxy_tags: Vec<ProxyTag>,
    pub tag: Option<String>
}
