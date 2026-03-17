use plural_export_macros::export_conversion;

use crate::models::{
    intermediary::IntermediaryExport,
    pluralkit::PluralKitExportV1
};


export_conversion! {
    PluralKitExportV1,
    Into<IntermediaryExport> |self| {
        todo!()
    }
    From<IntermediaryExport> |_intermediary| {
        todo!()
    }
}
