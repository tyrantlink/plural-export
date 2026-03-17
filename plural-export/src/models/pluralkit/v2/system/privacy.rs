use serde::{Deserialize, Serialize};

use crate::models::pluralkit::v2::enums::Privacy;


#[derive(Debug, Deserialize, Serialize)]
pub struct SystemPrivacy {
    pub avatar_privacy:        Privacy,
    pub banner_privacy:        Privacy,
    pub description_privacy:   Privacy,
    pub front_history_privacy: Privacy,
    pub front_privacy:         Privacy,
    pub group_list_privacy:    Privacy,
    pub member_list_privacy:   Privacy,
    pub name_privacy:          Privacy,
    pub pronoun_privacy:       Privacy
}
