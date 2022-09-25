use common::init_logger;
use serde::Deserialize;
use serde_xml_rs::{from_str, Deserializer};

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Project {
    name: String,

    #[serde(rename = "item", default)]
    items: Vec<Item>,
}

#[cfg(test)]
mod test {

    #[test]
    fn nested_collection() {
        init_logger();

        let s = r##"
            <project name="my_project">
                <item name="hello1" source="world1.rs" />
                <item name="hello2" source="world2.rs" />
            </project>
        "##;

        let project: Project = from_str(s).unwrap();

        assert_eq!(
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
        );

    }
}