use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Id(DateTime<Utc>);
impl Id
{
    pub fn new() -> Self
    {
        Self(Utc::now())
    }
}

/*
impl std::fmt::Debug for Id
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{:?}", self.0)
    }
}


impl FromStr for Id
{
    type Err = MumaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        Ok(Self(Instant::now (s)?))
    }
}


impl Serialize for Id
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer 
    {
        serializer.serialize_str(&self.0.to_string())
    }
}


impl<'de> Deserialize<'de> for Id
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        deserializer.deserialize_any(IdVisitor)
    }
}


struct IdVisitor;
impl<'de> Visitor<'de> for IdVisitor
{
    type Value = Id;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(formatter, "a uuid v4 string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error
    {
        match Id::from_str(&v)
        {
            Err(e) => Err(serde::de::Error::custom("UUID failed to parse")),
            Ok(id) => Ok(id),
        }
    }
}
*/
