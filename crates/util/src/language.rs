use inflector::cases::pascalcase::to_pascal_case;
pub use iso639_enum::{Err, Iso639, IsoCompat};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Language(pub Iso639);

impl FromStr for Language {
    type Err = iso639_enum::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Language(Iso639::from_iso639_3(&s.to_ascii_lowercase())?))
    }
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&to_pascal_case(self.0.iso639_3()))
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Language::from_str(&s).map_err(serde::de::Error::custom)
    }
}
