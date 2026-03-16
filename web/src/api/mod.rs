use dioxus::prelude::*;
use plural_export::Export;

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
        "pluralkit" => body.0.inner_into_pluralkit(),
        "pluralkit-v1" => body.0.inner_into_pluralkit_v1(),
        "pluralkit-v2" => body.0.inner_into_pluralkit_v2(),
        "tupperbox" => body.0.inner_into_tupperbox(),
        // TODO parsing in general should probably return a Result
        // TODO as well as, potentially, a log file
        _ => panic!("Invalid target format")
    }))
}
