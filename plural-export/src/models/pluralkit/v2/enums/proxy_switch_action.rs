use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxySwitchAction {
    /// if the current switch has 0 members, log a new switch
    /// with the currently proxied member;
    /// otherwise, add the member to the current switch
    Add,
    /// if the currently proxied member is not present
    /// in the current switch, log a new switch with this member
    New,
    /// do nothing
    Off
}
