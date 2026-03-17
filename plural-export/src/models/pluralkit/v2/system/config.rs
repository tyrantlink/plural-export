use serde::{Deserialize, Serialize};

use crate::models::pluralkit::v2::enums::{IdPaddingFormat, ProxySwitchAction};

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemConfig {
    pub card_show_color_hex: Option<bool>,
    /// whether the bot will match proxy tags matching
    /// only the case used in the trigger message
    pub case_sensitive_proxy_tags: bool,
    // ? undocumented in the pk System Settings Model
    pub description_templates: Vec<String>,
    /// whether groups created through the bot have
    /// privacy settings set to private by default
    pub group_default_private: bool,
    /// read-only, defaults to 250
    pub group_limit: u32,
    /// whether ids will be shown by the bot in uppercase
    pub hid_display_caps: bool,
    /// whether 6-character ids will be shown by the bot
    /// as two 3-character parts separated by a `-`
    pub hid_display_split: bool,
    /// whether the bot will pad 5-character ids in lists
    pub hid_list_padding: IdPaddingFormat,
    /// seconds after which latch autoproxy will
    /// timeout ([`None`] is 6 hours, 0 is "never")
    pub latch_timeout: Option<u32>,
    /// whether members created through the bot have
    /// privacy settings set to private by default
    pub member_default_private: bool,
    /// read-only, defaults to 1000
    pub member_limit: u32,
    /// format used for webhook names during proxying
    /// (defaults to `{name} {tag}`)
    pub name_format: Option<String>,
    /// whether proxied messages can be pinged using the 🔔 reaction
    pub pings_enabled: bool,
    /// whether the bot will show errors when proxying fails
    pub proxy_error_message_enabled: bool,
    /// switch action the bot will take when proxying
    pub proxy_switch: ProxySwitchAction,
    /// whether the bot shows the system's own private
    /// information without a `-private` flag
    pub show_private_info: bool,
    /// defaults to `UTC`
    pub timezone: String
}
