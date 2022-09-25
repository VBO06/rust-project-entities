use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn datefmt<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}

fn option_datefmt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "datefmt")] DateTime<Utc>);

    let v = Option::deserialize(deserializer)?;
    Ok(v.map(|Wrapper(a)| a))
}

#[derive(Deserialize, Debug)]
struct MyStruct {
    #[serde(default, deserialize_with = "option_datefmt")]
    expiration_date: Option<DateTime<Utc>>,
}

fn main() {
    let j = r#" {"expiration_date": null} "#;
    println!("{:?}", serde_json::from_str::<MyStruct>(j).unwrap());

    let j = r#" {"expiration_date": "2017-02-16 21:54:30"} "#;
    println!("{:?}", serde_json::from_str::<MyStruct>(j).unwrap());
}