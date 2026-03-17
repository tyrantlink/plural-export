use dioxus::{CapturedError, prelude::*};
use plural_export::{
    Export,
    models::{PluralKitExportV1, PluralKitExportV2, TupperboxExport}
};

use crate::api::dioxus_fullstack::Json;

#[post("/api/v1/convert/{target_format}")]
#[expect(
    clippy::unused_async,
    reason = "this allow should just be part of the macro"
)]
pub async fn post_convert(
    target_format: String,
    body: Json<Export>
) -> Result<Json<Export>> {
    Ok(Json(match target_format.as_str() {
        "pluralkit" => body.0.transmute::<PluralKitExportV2>(&mut Vec::new()),
        "pluralkit-v1" => {
            body.0.transmute::<PluralKitExportV1>(&mut Vec::new())
        }
        "pluralkit-v2" => {
            body.0.transmute::<PluralKitExportV2>(&mut Vec::new())
        }
        "tupperbox" => body.0.transmute::<TupperboxExport>(&mut Vec::new()),
        // TODO parsing in general should probably return a Result
        // TODO as well as, potentially, a log file
        _ => return Err(CapturedError::msg("Invalid target format"))
    }))
}
