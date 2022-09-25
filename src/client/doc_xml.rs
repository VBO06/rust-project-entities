use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PlateAppearance {
    #[serde(rename = "$value")]
    events: Vec<Event>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Event {
    Pitch(Pitch),
    Runner(Runner),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Pitch {
    speed: u32,
    r#type: PitchType,
    outcome: PitchOutcome,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum PitchType { FourSeam, TwoSeam, Changeup, Cutter, Curve, Slider, Knuckle, Pitchout }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum PitchOutcome { Ball, Strike, Hit }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Runner {
    from: Base, to: Option<Base>, outcome: RunnerOutcome,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Base { First, Second, Third, Home }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum RunnerOutcome { Steal, Caught, PickOff }

fn main() {
    let document = r#"
        <plate-appearance>
          <pitch speed="95" type="FourSeam" outcome="Ball" />
          <pitch speed="91" type="FourSeam" outcome="Strike" />
          <pitch speed="85" type="Changeup" outcome="Ball" />
          <runner from="First" to="Second" outcome="Steal" />
          <pitch speed="89" type="Slider" outcome="Strike" />
          <pitch speed="88" type="Curve" outcome="Hit" />
        </plate-appearance>"#;
    let plate_appearance: PlateAppearance = from_str(document).unwrap();
    println!("{:?}", plate_appearance);
    assert_eq!(plate_appearance.events[0], Event::Pitch(Pitch { speed: 95, r#type: PitchType::FourSeam, outcome: PitchOutcome::Ball }));
}
