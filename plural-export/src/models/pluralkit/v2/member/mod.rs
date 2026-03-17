mod privacy;
mod proxy_tag;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use self::{privacy::MemberPrivacy, proxy_tag::ProxyTag};
use crate::utils::Id;


#[derive(Debug, Deserialize, Serialize)]
pub struct Member {
    pub autoproxy_enabled: Option<bool>,
    /// 256-character limit, must be a publicly-accessible URL
    pub avatar_url: Option<String>,
    /// 256-character limit, must be a publicly-accessible URL
    pub banner: Option<String>,
    /// `YYYY-MM-DD` format, 0004 hides the year
    pub birthday: Option<String>,
    /// 6-character hex code, no # at the beginning
    pub color: Option<String>,
    pub created: Option<DateTime<Utc>>,
    /// 1000-character limit
    pub description: Option<String>,
    /// 100-character limit
    pub display_name: Option<String>,
    pub id: Id<Self>,
    pub keep_proxy: bool,
    pub last_message_timestamp: Option<DateTime<Utc>>,
    pub message_count: Option<u64>,
    /// 100-character limit
    pub name: String,
    pub privacy: Option<MemberPrivacy>,
    pub pronouns: Option<String>,
    pub proxy_tags: Vec<ProxyTag>,
    pub tts: bool,
    pub uuid: String,
    /// 256-character limit, must be a publicly-accessible URL
    pub webhook_avatar_url: Option<String>
}
