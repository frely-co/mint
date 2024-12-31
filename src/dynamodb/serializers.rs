
use bytes::Bytes;
use serde::{ser::SerializeSeq, Deserialize, Serializer};

pub fn serialize_list<S>(bytes_list: &Option<Vec<Bytes>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(bytes_list) = bytes_list {
        let encoded_list: Vec<String> = bytes_list.iter().map(|b| base64::encode(b)).collect();
        let mut seq = serializer.serialize_seq(Some(encoded_list.len()))?;
        for encoded in encoded_list {
            seq.serialize_element(&encoded)?;
        }
        seq.end()
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize_list<'de, D>(deserializer: D) -> Result<Option<Vec<Bytes>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Vec<String> = Vec::deserialize(deserializer)?;
    let result: Result<Vec<Bytes>, _> = s
        .into_iter()
        .map(|s| {
            base64::decode(s)
                .map(Bytes::from)
                .map_err(serde::de::Error::custom)
        })
        .collect();
    result.map(Some)
}

pub fn serialize<S>(bytes: &Option<Bytes>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match bytes {
        Some(b) => serializer.serialize_str(&base64::encode(b)),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Bytes>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => base64::decode(s)
            .map(Bytes::from)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
