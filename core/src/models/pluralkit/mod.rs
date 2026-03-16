pub mod v1;
pub mod v2;

use serde::{Deserialize, Deserializer, Serialize, de::Error};
use serde_json::Value;

#[expect(
    clippy::large_enum_variant,
    reason = "this is a poc i can deal with it later™"
)]
#[derive(Debug, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum PluralKitExport {
    V1(v1::Export),
    V2(v2::Export)
}

// TODO this serializes into Value first and i DON'T LIKE IT
// TODO MAKE THIS LESS BAD
impl<'de> Deserialize<'de> for PluralKitExport {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D
    ) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;

        let version = value
            .get("version")
            .and_then(Value::as_u64)
            .ok_or_else(|| D::Error::missing_field("version"))?;

        match version {
            1 => Ok(Self::V1(
                v1::Export::deserialize(value).map_err(D::Error::custom)?
            )),
            2 => Ok(Self::V2(
                v2::Export::deserialize(value).map_err(D::Error::custom)?
            )),
            version => {
                Err(D::Error::custom(format!("unknown version: {version}")))
            }
        }
    }
}

// ? probably impl read functions for version agnostic reading
