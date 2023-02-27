use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn deserialize_i64_from_string_or_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    if value.is_string() {
        let val = value.as_str().unwrap().parse().map_err(|_| {
            serde::de::Error::custom(
                "unexpected value, string does not contain a well formatted i64",
            )
        })?;
        Ok(val)
    } else {
        Ok(value
            .as_i64()
            .ok_or_else(|| serde::de::Error::custom("unexpected value, not i64 or string"))?)
    }
}
