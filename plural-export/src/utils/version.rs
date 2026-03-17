use serde::{
    Deserialize,
    Serialize,
    de::{Deserializer, Error, Unexpected},
    ser::Serializer
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntVersion<const LITERAL: u32>;


impl<'de, const LITERAL: u32> Deserialize<'de> for IntVersion<LITERAL> {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D
    ) -> Result<Self, D::Error> {
        let value = u32::deserialize(deserializer)?;

        if value != LITERAL {
            return Err(D::Error::invalid_value(
                Unexpected::Unsigned(value as u64),
                &"unknown version"
            ));
        }

        Ok(Self)
    }
}

impl<const LITERAL: u32> Serialize for IntVersion<LITERAL> {
    fn serialize<S: Serializer>(
        &self,
        serializer: S
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(LITERAL)
    }
}
