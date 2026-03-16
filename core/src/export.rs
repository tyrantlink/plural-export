use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult}
};

use serde::{Deserialize, Serialize};

use crate::{
    ExportConversion,
    models::{pluralkit::PluralKitExport, tupperbox::TupperboxExport}
};

#[expect(
    clippy::large_enum_variant,
    reason = "this is a poc i can deal with it later™"
)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Export {
    PluralKit(PluralKitExport),
    Tupperbox(TupperboxExport)
}

pub enum ExportType {
    PluralKitV1,
    PluralKitV2,
    Tupperbox
}

impl Export {
    #[must_use]
    pub fn inner_into_pluralkit(self) -> Self {
        Self::PluralKit(self.into_pluralkit())
    }

    #[must_use]
    pub fn inner_into_pluralkit_v1(self) -> Self {
        Self::PluralKit(PluralKitExport::V1(self.into_pluralkit_v1()))
    }

    #[must_use]
    pub fn inner_into_pluralkit_v2(self) -> Self {
        Self::PluralKit(PluralKitExport::V2(self.into_pluralkit_v2()))
    }

    #[must_use]
    pub fn inner_into_tupperbox(self) -> Self {
        Self::Tupperbox(self.into_tupperbox())
    }

    #[must_use]
    pub const fn r#type(&self) -> ExportType {
        match self {
            Self::PluralKit(PluralKitExport::V1(_)) => ExportType::PluralKitV1,
            Self::PluralKit(PluralKitExport::V2(_)) => ExportType::PluralKitV2,
            Self::Tupperbox(_) => ExportType::Tupperbox
        }
    }
}

impl ExportType {
    #[must_use]
    pub fn list() -> HashMap<&'static str, Self> {
        HashMap::from([
            ("PluralKit v1", Self::PluralKitV1),
            ("PluralKit v2", Self::PluralKitV2),
            ("Tupperbox", Self::Tupperbox)
        ])
    }
}

impl Display for Export {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.r#type(), f)
    }
}

impl Display for ExportType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::PluralKitV1 => f.write_str("PluralKit v1"),
            Self::PluralKitV2 => f.write_str("PluralKit v2"),
            Self::Tupperbox => f.write_str("Tupperbox")
        }
    }
}
