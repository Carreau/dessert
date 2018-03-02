#[macro_use]
extern crate dessert_derive;
extern crate dessert;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use dessert::ViaDeserialize;


#[derive(ViaDeserialize, Debug)]
#[via(Intermediate)]
struct FrenchToast {
    ingredient: String,
}

#[derive(Deserialize)]
struct Intermediate {
    val: String,
}

impl From<Intermediate> for FrenchToast {
    fn from(b: Intermediate) -> Self {
        Self { ingredient: b.val }
    }
}

fn main() {
    let serialized_string= "{\"val\":\"Butter\"}";
    let v: FrenchToast = serde_json::from_str(serialized_string).unwrap();
    println!("Serialized form: {}", serialized_string);
    println!("Debug format : {:?}", v)
}
