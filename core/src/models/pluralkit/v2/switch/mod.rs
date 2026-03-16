use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Id, models::pluralkit::v2::Member};

#[derive(Debug, Deserialize, Serialize)]
pub struct Switch {
    // ? technically *can* be Vec<Member> but
    // ? i think it's only ids in exports
    pub members:   Vec<Id<Member>>,
    pub timestamp: DateTime<Utc>
}
