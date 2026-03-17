mod config;
mod privacy;

use serde::{Deserialize, Serialize};

pub use self::{config::SystemConfig, privacy::SystemPrivacy};

/// Not modeled for exports, only used for the [`Id`] type
///
/// [`Id`]: crate::Id
#[derive(Debug, Deserialize, Serialize)]
pub struct System;
