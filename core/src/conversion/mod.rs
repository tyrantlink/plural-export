mod pluralkit;
mod tupperbox;


use crate::{
    export::Export,
    models::{
        intermediary::IntermediaryExport,
        pluralkit::{
            PluralKitExport,
            v1::Export as PluralKitExportV1,
            v2::Export as PluralKitExportV2
        },
        tupperbox::TupperboxExport
    }
};


pub trait ExportConversion: Sized {
    fn into_intermediary(self) -> IntermediaryExport;

    /// alias for [`Self::into_pluralkit_v2`]
    fn into_pluralkit(self) -> PluralKitExport {
        PluralKitExport::V2(self.into_pluralkit_v2())
    }

    fn into_pluralkit_v1(self) -> PluralKitExportV1 {
        PluralKitExportV1::from(self.into_intermediary())
    }

    fn into_pluralkit_v2(self) -> PluralKitExportV2 {
        PluralKitExportV2::from(self.into_intermediary())
    }

    fn into_tupperbox(self) -> TupperboxExport {
        TupperboxExport::from(self.into_intermediary())
    }
}

impl ExportConversion for Export {
    fn into_intermediary(self) -> IntermediaryExport {
        match self {
            Self::PluralKit(plural_kit) => plural_kit.into_intermediary(),
            Self::Tupperbox(tupperbox) => tupperbox.into_intermediary()
        }
    }
}
