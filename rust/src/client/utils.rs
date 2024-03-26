use crate::client::types::ClientResult;

pub fn from_json_str<T: for<'de> serde::Deserialize<'de>>(json: &str) -> ClientResult<T> {
    let mut de = serde_json::Deserializer::from_str(json);
    let de = serde_path_to_error::deserialize(&mut de)?;
    Ok(de)
}

pub fn from_json_value<T: for<'de> serde::Deserialize<'de>>(
    json: &serde_json::Value,
) -> ClientResult<T> {
    let de = serde_path_to_error::deserialize(json)?;
    Ok(de)
}
