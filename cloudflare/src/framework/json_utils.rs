use serde::de;

/// Serializes a vector of bytes as a base64 encoded string.
pub fn serialize_base64_str<S>(val: &[u8], serializer: S) -> serde::export::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&base64::encode(val))
}

/// Deserializes a base64 encoded string as a vector of bytes.
pub fn deserialize_base64_str<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Base64Visitor;

    impl<'de> de::Visitor<'de> for Base64Visitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(formatter, "base64 ASCII text")
        }

        fn visit_str<E>(self, v: &str) -> serde::export::Result<Self::Value, E>
        where
            E: de::Error,
        {
            base64::decode(v).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_str(Base64Visitor)
}
