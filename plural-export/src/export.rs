use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult}
};

use serde::{Deserialize, Serialize};

use crate::{
    conversion::LoggedFrom,
    models::{
        pluralkit::{PluralKitExportV1, PluralKitExportV2},
        tupperbox::TupperboxExport
    }
};

#[expect(
    clippy::large_enum_variant,
    reason = "this is a poc i can deal with it later™"
)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Export {
    PluralKitV1(PluralKitExportV1),
    PluralKitV2(PluralKitExportV2),
    Tupperbox(TupperboxExport)
}

pub enum ExportType {
    PluralKitV1,
    PluralKitV2,
    Tupperbox
}

impl Export {
    #[must_use]
    #[expect(
        private_bounds,
        reason = "intentionally keeping LoggedFrom private"
    )]
    pub fn transmute<
        Target: LoggedFrom<PluralKitExportV1>
            + LoggedFrom<PluralKitExportV2>
            + LoggedFrom<TupperboxExport>
    >(
        self,
        log: &mut Vec<String>
    ) -> Self
    where
        Self: From<Target>
    {
        match self {
            Self::PluralKitV1(value) => {
                Self::from(Target::logged_from(value, log))
            }
            Self::PluralKitV2(value) => {
                Self::from(Target::logged_from(value, log))
            }
            Self::Tupperbox(value) => {
                Self::from(Target::logged_from(value, log))
            }
        }
    }

    #[must_use]
    pub const fn r#type(&self) -> ExportType {
        match self {
            Self::PluralKitV1(_) => ExportType::PluralKitV1,
            Self::PluralKitV2(_) => ExportType::PluralKitV2,
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

impl From<PluralKitExportV1> for Export {
    fn from(value: PluralKitExportV1) -> Self {
        Self::PluralKitV1(value)
    }
}

impl From<PluralKitExportV2> for Export {
    fn from(value: PluralKitExportV2) -> Self {
        Self::PluralKitV2(value)
    }
}

impl From<TupperboxExport> for Export {
    fn from(value: TupperboxExport) -> Self {
        Self::Tupperbox(value)
    }
}
