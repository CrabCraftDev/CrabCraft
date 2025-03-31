use crate::error::TextError;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub const MINECRAFT_NAMESPACE: &str = "minecraft";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct NamespacedKey {
    pub namespace: String,
    pub value: String,
}

impl NamespacedKey {
    pub fn new(namespace: &str, value: &str) -> Result<NamespacedKey, TextError> {
        if !NamespacedKey::is_valid_namespace(namespace) || !NamespacedKey::is_valid_value(value) {
            return Err(TextError::InvalidNamespacedKeyFormat(format!("{namespace}:{value}")));
        }

        Ok(NamespacedKey {
            namespace: namespace.to_string(),
            value: value.to_string(),
        })
    }

    pub fn minecraft(value: &str) -> Result<NamespacedKey, TextError> {
        NamespacedKey::new(MINECRAFT_NAMESPACE, value)
    }

    fn is_valid_namespace(namespace: &str) -> bool {
        namespace
            .chars()
            .all(|char| char.is_ascii_lowercase() || char.is_ascii_digit() || char == '.' || char == '-' || char == '_')
    }

    fn is_valid_value(value: &str) -> bool {
        value.chars().all(|char| {
            char.is_ascii_lowercase()
                || char.is_ascii_digit()
                || char == '.'
                || char == '-'
                || char == '_'
                || char == '/'
        })
    }
}

impl TryFrom<&str> for NamespacedKey {
    type Error = TextError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(':');
        // We take 3 elements to catch the case where there is more then one colon in the string
        match (split.next(), split.next(), split.next()) {
            (Some(value), None, None) => Ok(NamespacedKey::minecraft(value)?),
            (Some(namespace), Some(value), None) => Ok(NamespacedKey::new(namespace, value)?),
            _ => Err(TextError::InvalidNamespacedKeyFormat(value.to_owned())),
        }
    }
}

impl FromStr for NamespacedKey {
    type Err = TextError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NamespacedKey::try_from(s)
    }
}

impl Display for NamespacedKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}:{}", self.namespace, self.value)
    }
}

impl Serialize for NamespacedKey {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(format!("{}:{}", self.namespace, self.value).as_str())
    }
}
struct StrVisitor;
impl Visitor<'_> for StrVisitor {
    type Value = String;
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("A string")
    }
    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(value.to_string())
    }
    fn visit_string<E: Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(value)
    }
}
impl<'de> Deserialize<'de> for NamespacedKey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::try_from(&*deserializer.deserialize_str(StrVisitor).unwrap_or("".to_string())).unwrap())
    }
}
