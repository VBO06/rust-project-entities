#![feature(proc_macro_hygiene, decl_macro)]

use serde::{Deserialize, Deserializer, Serialize};
use std::*;
use serde_with::*;
use rocket::post;
use chrono::prelude::*;
use rocket::Request;
use rocket::Response;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use std::io::Cursor;
use rocket::response::{self, Responder};
use rocket::http::ContentType;

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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[allow(non_snake_case)]
#[serde(rename = "G1_1000")]
pub struct G1_1000 {
    #[serde(rename = "Entite", default)]
    items: Vec<Entite>,
}

impl<'r> Responder<'r> for G1_1000 {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{:#?}", self.items)))
            .header(ContentType::new("application", "x-person"))
            .ok()
    }
}

impl G1_1000 {

    fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }
    
    /// Add a new bill. If a bill with the same name exists, it is overwritten.
    fn add(&mut self, entite: Entite) {
        // We need to clone the bill name, since the String type cannot be implicitly copied.
        // Without the clone, the name would get moved into the 'key' portion of the hashmap,
        // and therefore would be moved out of the bill struct.
        self.items.push(entite.clone());
    }

    fn get_all(&self) -> Vec<&Entite> {
        let mut entites = vec![];
        // Iterate through each value of the bill hashmap, ignoring the keys.
        for entite in self.items.iter() {
            // Slight change made after the video was created: We are using
            // a borrow here to make the program more efficient. When iterating
            // using .values(), the value is borrowed automatically.
            entites.push(entite);
        }
        entites
    }

}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TEST {
    user_id: i32,
    name: String,
}

#[post("/entites", format = "application/json", data = "<entites>")]
pub fn entites(entites: Json<G1_1000>) -> JsonValue {
    println!("From Server : {:?}", entites);
    let entite_to_respond = entites.clone();
    return json!(entite_to_respond);
}

#[post("/test", format = "application/json", data = "<tests>")]
pub fn new_test(tests: Json<TEST>) -> Json<TEST> {
    println!("From Server : {:?}", tests);
    return tests;
}

fn main() {
    rocket::ignite()
            .mount("/",routes![entites,new_test]).launch();
}


pub fn insert_entite(entites: Json<G1_1000>)  {
        println!("JSON {:?}", entites);
}


pub fn get_all_entite(entites: Json<G1_1000>)  {
    println!("JSON {:?}", entites);
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
