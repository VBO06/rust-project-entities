#![feature(proc_macro_hygiene, decl_macro)]

use serde::{Deserialize, Deserializer, Serialize};
use xml::reader::{EventReader, XmlEvent};
use serde_xml_rs::{from_str, to_string};
use chrono::prelude::*;
use chrono::DateTime;
use chrono::Utc;
use std::fs::File;
use std::*;
use serde_with::*;
use std::str;
use std::io::{self,prelude::*,BufReader,Write};
use tokio;
extern crate serde;
extern crate serde_with;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;

// const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const FORMAT: &str = "%Y-%m-%d";
const ENTITE_NS: &str = "{http://www.gsf.fr/UNIVERS/entiteGSF}";

const ACCEPT: &str = "Accept";
const CONTENT_TYPE: &str = "Content-Type";

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
struct Entite {
    IdSource: String,
    NomInterne: String,
    NomCommercial: String,
    TypeEntite: String,
    SousTypeEntite: String,
    #[serde(default, deserialize_with = "test_naive_date_from_string")]
    DateDebut: NaiveDate,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
#[serde(rename = "G1_1000")]
struct G1_1000 {
    #[serde(rename = "Entite", default)]
    items: Vec<Entite>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct TEST {
    user_id: i32,
    name: String,
}

fn test_date_time_from_string<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    println!("date from file : {:?}", &s);

        match Utc.datetime_from_str(&s, FORMAT) {
            Ok(date) => {
                println!("bingo: {}", date);
                return Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
            }
            Err(e) => {
                println!("woops: {}", e);
                return Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
            }
        };
}

fn test_naive_date_from_string<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    println!("date from file : {:?}", &s);

        match NaiveDate::parse_from_str(&s, FORMAT) {
            Ok(date) => {
                println!("bingo: {}", date);
                return NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
            }
            Err(e) => {
                println!("woops: {}", e);
                return NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
            }
        };
}

fn read_file() -> String {
    let file = File::open("C:\\Users\\vboisnier_prestatair\\EntiteGSF_Rust.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut xml_str = String::new();
    let not_namespace = vec![ "xmlns".to_string(), "xml".to_string()];

    let mut index = 1;

    for event in parser {
        
        match event {
            Ok(XmlEvent::StartElement { name, attributes , namespace }) => {
                let mut namespace_file = String::new();
                let mut namespace_to_save = String::new();
                let mut format_first_node = String::new();
                for (key, value) in &namespace {
                    if key.ne(&not_namespace[0].to_string()) && key.ne(&not_namespace[1].to_string()) {
                        namespace_file = namespace.get(key).unwrap().to_string();  
                        namespace_to_save = key.to_string();                    
                    }
                }
                let _c = format!("<{}>", name);

                let mut format_namespace = String::new();
                format_namespace.push_str(&"{".to_owned());
                format_namespace.push_str(&namespace_file);
                format_namespace.push_str(&"}".to_owned());
                let _c = str::replace(&_c, &format_namespace, "");

                if index == 2 {
                    let _c = str::replace(&_c, ">", "");
                    format_first_node.push_str(&_c);
                    format_first_node.push_str(" xmlns:");
                    format_first_node.push_str(&namespace_to_save);
                    format_first_node.push_str("=");
                    let string1 = r#"""#; // " 
                    format_first_node.push_str(string1);
                    format_first_node.push_str(&namespace_file);
                    format_first_node.push_str(string1);
                    format_first_node.push_str(">");
                    xml_str.push_str(&format_first_node);
                } else {
                    xml_str.push_str(&_c);
                }

            }
            Ok(XmlEvent::Characters(value)) => {
                let _c = format!("{}", value);
                let _c = str::replace(&_c, ENTITE_NS, "");
                xml_str.push_str(&_c);
            }
            Ok(XmlEvent::EndElement { name }) => {
                let _c = format!("</{}>", name);
                let _c = str::replace(&_c, ENTITE_NS, "");
                xml_str.push_str(&_c);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }

        index += 1;
    }
    // println!("String : {:?}", &xml_str);
    return xml_str.to_string();
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml_str = read_file();
    let entites= nested_collection(&xml_str);

    let handle = tokio::spawn(async move {
        // Process each socket concurrently 
        let client = reqwest::Client::new();
        let client = client.clone();
        let response = client.post("http:/172.20.0.36:9225/entites")
        .header("Content-Type", "application/json")
        .json(&entites)
        .send()
        .await.unwrap()
        .json::<G1_1000>()
        .await.unwrap();
        
        let s = serde_json::to_string(&response);
        let v: G1_1000 = serde_json::from_str(&s.unwrap()).unwrap();
        println!("Object receive from API : {:#?}", v);

    });

    handle.await?;

    Ok(())
}


 fn nested_collection(xml_str: &str) -> G1_1000 {
    let entites: G1_1000 = from_str(xml_str).unwrap();

    for ent in &entites.items {
        println!("Key {:?} Value : {:?}", ent.IdSource, ent.NomCommercial);
    }

    let entt:Vec<_> = entites.items.iter().filter(|entit| entit.NomCommercial.eq("Test")).collect();
    println!("entt :   {:?}", &entt);

    return entites;

}



