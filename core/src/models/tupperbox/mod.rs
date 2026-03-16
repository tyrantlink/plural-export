pub mod group;
pub mod tupper;

use serde::{Deserialize, Serialize};

pub use self::{group::Group, tupper::Tupper};


#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TupperboxExport {
    pub groups:  Vec<Group>,
    pub tuppers: Vec<Tupper>
}

impl TupperboxExport {
    /// Tupperbox doesn't include the user id in the export
    /// so our best option is to find it from avatar urls
    #[must_use]
    pub fn find_user_id(&self) -> Option<u64> {
        for tupper in &self.tuppers {
            // ? i don't wanna use regex it's too heavyyyyyy
            // ? but this is like, gross
            let Some(user_id) = tupper
                .avatar_url
                .as_deref()
                .and_then(|url| {
                    url.strip_prefix("https://cdn.tupperbox.app/pfp/")
                })
                .and_then(|path| path.split('/').next())
                .and_then(|id| id.parse::<u64>().ok())
            else {
                continue
            };

            return Some(user_id);
        }

        None
    }
}
