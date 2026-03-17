mod privacy;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use self::privacy::GroupPrivacy;
use crate::{models::pluralkit::v2::Member, utils::Id};


#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    /// 256-character limit, must be a publicly-accessible URL
    pub banner:       Option<String>,
    /// 6-character hex code, no # at the beginning
    pub color:        Option<String>,
    pub created:      Option<DateTime<Utc>>,
    /// 1000-character limit
    pub description:  Option<String>,
    /// 100-character limit
    pub display_name: Option<String>,
    /// 256-character limit, must be a publicly-accessible URL
    pub icon:         Option<String>,
    pub id:           Id<Self>,
    pub members:      Vec<Id<Member>>,
    /// 100-character limit
    pub name:         String,
    pub privacy:      Option<GroupPrivacy>,
    pub uuid:         String
}
