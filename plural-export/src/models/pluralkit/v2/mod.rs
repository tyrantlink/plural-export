pub mod enums;
pub mod group;
pub mod member;
pub mod switch;
pub mod system;


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use self::{
    enums::Privacy,
    group::Group,
    member::Member,
    switch::Switch,
    system::{System, SystemConfig, SystemPrivacy}
};
use crate::utils::{Id, IntVersion};

#[derive(Debug, Deserialize, Serialize)]
pub struct PluralKitExportV2 {
    pub accounts:    Vec<i64>,
    /// 256-character limit, must be a publicly-accessible URL
    pub avatar_url:  Option<String>,
    /// 256-character limit, must be a publicly-accessible URL
    pub banner:      Option<String>,
    /// 6-character hex code, no # at the beginning
    pub color:       Option<String>,
    pub config:      SystemConfig,
    pub created:     DateTime<Utc>,
    /// 1000-character limit
    pub description: Option<String>,
    pub groups:      Vec<Group>,
    pub id:          Id<System>,
    pub members:     Vec<Member>,
    /// 100-character limit
    pub name:        Option<String>,
    pub privacy:     Option<SystemPrivacy>,
    /// 100-character limit
    pub pronouns:    Option<String>,
    pub switches:    Vec<Switch>,
    /// 79-character limit
    pub tag:         Option<String>,
    pub uuid:        String,
    pub version:     IntVersion<2>,
    pub webhook_url: Option<String>
}
