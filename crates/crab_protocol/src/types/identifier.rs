use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Identifier {
    pub namespace: String,
    pub value: String,
}

impl Identifier {
    pub fn new(namespace: &str, value: &str) -> Result<Identifier, ProtocolError> {
        if !Identifier::is_valid_namespace(namespace) || !Identifier::is_valid_value(value) {
            return Err(ProtocolError::InvalidIdentifier(format!("{namespace}:{value}")));
        }

        Ok(Identifier {
            namespace: namespace.to_string(),
            value: value.to_string(),
        })
    }

    pub fn minecraft(value: &str) -> Result<Identifier, ProtocolError> {
        Identifier::new("minecraft", value)
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

impl TryFrom<&str> for Identifier {
    type Error = ProtocolError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split(':').collect();
        match split.len() {
            1 => Ok(Identifier::new("minecraft", value)?),
            2 => Ok(Identifier::new(split[0], split[1])?),
            _ => Err(ProtocolError::InvalidIdentifier(value.to_owned())),
        }
    }
}

impl FromStr for Identifier {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Identifier::try_from(s)
    }
}

impl Display for Identifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}:{}", self.namespace, self.value)
    }
}

impl ProtocolSerializer for Identifier {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        ProtocolSerializer::serialize(&self.to_string(), buf)?;
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let value: String = ProtocolSerializer::deserialize(buf)?;
        value.parse()
    }
}
