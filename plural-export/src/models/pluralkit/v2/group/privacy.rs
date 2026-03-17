use serde::{Deserialize, Serialize};

use crate::models::pluralkit::v2::enums::Privacy;


#[derive(Debug, Deserialize, Serialize)]
pub struct GroupPrivacy {
    pub banner_privacy:      Privacy,
    pub description_privacy: Privacy,
    pub icon_privacy:        Privacy,
    pub list_privacy:        Privacy,
    pub metadata_privacy:    Privacy,
    pub name_privacy:        Privacy,
    pub visibility:          Privacy
}
