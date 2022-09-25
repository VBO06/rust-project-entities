#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

#[derive(Deserialize, Debug)]
struct Note {
    name: String,
    body: Body,
}

#[derive(Deserialize, Debug)]
struct Body {
    name: String,
    #[serde(rename="layer")]
    layers: Vec<Layer>,
}

#[derive(Deserialize, Debug)]
struct Layer {
    content_type: String,
    count: u8,
    data: Vec<Data>,
}

#[derive(Deserialize, Debug)]
struct Data {
    id: u8,
    #[serde(rename="$value")]
    content: String,
}

fn main() {
    let note: Note = serde_xml_rs::deserialize(r##"
<?xml version="1.0" encoding="UTF-8"?>
<note name="title">
  <body name="main_body">
    <layer content_type="something" count="99">
      <data id="13">
        Datacontent
      </data>
    </layer>
  </body>
</note>
    "##.as_bytes()).unwrap();
    println!("NOTE :  {:#?}", note);
}