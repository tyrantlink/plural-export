use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IdPaddingFormat {
    /// add a padding space to the left of 5-character ids in lists
    Left,
    /// do not pad 5-character ids
    Off,
    /// add a padding space to the right of 5-character ids in lists
    Right
}
