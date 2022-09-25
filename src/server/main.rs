#![feature(proc_macro_hygiene, decl_macro)]

use serde::{Deserialize, Deserializer, Serialize};
use xml::reader::{EventReader, XmlEvent};
use serde::de::Error;
use serde_xml_rs::{from_str, to_string};
use std::fs::File;
use std::*;
use serde_with::*;
use rocket::post;
use chrono::prelude::*;
use chrono::DateTime;
use chrono::Utc;

use rocket::request::FlashMessage;
use rocket::response::Redirect;
use std::collections::HashMap;
use rocket::Request;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde_json::Value;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_with;
extern crate serde_xml_rs;
extern crate chrono;
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

extern crate serde_json;

use rocket::routes;

const FORMAT: &str = "%Y-%m-%d";

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct Entite {
    IdSource: String,
    NomInterne: String,
    NomCommercial: String,
    TypeEntite: String,
    SousTypeEntite: String,
    #[serde(default, deserialize_with = "test_naive_date_from_string")]
    DateDebut: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_snake_case)]
#[serde(rename = "G1_XXXX_XXXX")]
pub struct G1_XXXX_XXXX {
    #[serde(rename = "Entite", default)]
    items: Vec<Entite>,
}

#[post("/entites", format = "application/json", data = "<entites>")]
pub fn entites(entites: Json<G1_XXXX_XXXX>) {
    println!("Entites : {:?}", entites);
}

fn main() {
    rocket::ignite()
            .mount("/entites",routes![entites]).launch();
}


pub fn insert_entite_gsf(entitesGSF: Json<G1_5005d_EntiteOperationnelle>)  {
        println!("JSON {:?}", entitesGSF);
}


pub fn get_all_entiteGSF(entitesGSF: Json<G1_5005d_EntiteOperationnelle>)  {
    println!("JSON {:?}", entitesGSF);
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