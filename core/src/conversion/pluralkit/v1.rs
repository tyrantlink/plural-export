use super::ExportConversion;
use crate::models::{
    intermediary::IntermediaryExport,
    pluralkit::v1::Export as PluralKitExportV1
};


impl ExportConversion for PluralKitExportV1 {
    fn into_intermediary(self) -> IntermediaryExport {
        todo!();
    }
}

impl From<IntermediaryExport> for PluralKitExportV1 {
    fn from(value: IntermediaryExport) -> Self {
        todo!();
    }
}
