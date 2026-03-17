use chrono::{DateTime, Utc};

use super::{Image, ProxyTag};

#[derive(Debug)]
pub struct Member {
    pub avatar: Option<Image>,
    pub banner: Option<Image>,
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
