mod v1;
mod v2;

use super::ExportConversion;
use crate::models::{
    intermediary::IntermediaryExport,
    pluralkit::PluralKitExport
};


impl ExportConversion for PluralKitExport {
    fn into_intermediary(self) -> IntermediaryExport {
        match self {
            Self::V1(v1) => v1.into_intermediary(),
            Self::V2(v2) => v2.into_intermediary()
        }
    }
}
