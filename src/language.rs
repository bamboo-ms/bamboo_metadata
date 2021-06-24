use inflector::cases::pascalcase::to_pascal_case;
use iso639_enum::{Iso639, IsoCompat};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Eq)]
pub struct Language(pub Iso639);

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
        let iso = match Iso639::from_iso639_3(&s.to_ascii_lowercase()) {
            Some(v) => v,
            None => panic!(), // TODO: Proper error handling
        };
        Ok(Language(iso))
    }
}
