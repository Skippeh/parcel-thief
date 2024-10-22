use serde::{Deserialize, Deserializer, Serializer};
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

pub fn deserialize_bool_from_number<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let num = i8::deserialize(deserializer)?;

    match num {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(serde::de::Error::custom(
            "unexpected value, expected 0 or 1.",
        )),
    }
}

pub fn serialize_bool_to_number<S>(val: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let number = match val {
        true => 1,
        false => 0,
    };

    serializer.serialize_u8(number)
}
