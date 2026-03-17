use serde::{Deserialize, Serialize};

use crate::models::pluralkit::v2::enums::Privacy;


#[derive(Debug, Deserialize, Serialize)]
pub struct MemberPrivacy {
    pub avatar_privacy:      Privacy,
    pub banner_privacy:      Privacy,
    pub birthday_privacy:    Privacy,
    pub description_privacy: Privacy,
    pub metadata_privacy:    Privacy,
    pub name_privacy:        Privacy,
    pub pronoun_privacy:     Privacy,
    pub proxy_privacy:       Privacy,
    pub visibility:          Privacy
}
