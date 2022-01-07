use std::{str::FromStr, num::ParseIntError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Snowflake(#[serde(with = "serde_snowflake")] u64);

mod serde_snowflake {
    use serde::{
        de::{ Deserializer, Error },
        Deserialize, Serializer
    };

    pub fn serialize<S>(snowflake: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&snowflake.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|_| Error::custom("invalid snowflake id"))
    }
}

impl ToString for Snowflake {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}

impl FromStr for Snowflake {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.parse::<u64>() {
            Ok(u) => Ok(Snowflake(u)),
            Err(e) => Err(e)
        }
    }
}

impl Into<Snowflake> for &str {
    fn into(self) -> Snowflake {
        let u = self.parse::<u64>().unwrap();
        return Snowflake(u);
    }
}