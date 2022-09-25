use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml_rs;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Project {
    #[serde(rename = "item", default)]
    items: Vec<Item>,
}

fn main() {
    nested_collection();
}

fn nested_collection() {

    let s = r##"<project><item><name>hello1</name><source>world1.rs</source></item></project>"##;

    let project: Project = from_str(s).unwrap();
    println!("{:?}", project);

    /*assert_eq!(
        project,
        Project {
            name: "my_project".to_string(),
            items: vec![
                Item {
                    name: "hello1".to_string(),
                    source: "world1.rs".to_string(),
                },
                Item {
                    name: "hello2".to_string(),
                    source: "world2.rs".to_string(),
                },
            ],
        }
    );*/
}