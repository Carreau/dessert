extern crate dessert;
#[macro_use]
extern crate dessert_derive;
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

#[test]
fn test_main() -> () {
    main()
}

fn main() -> () {
    let serialized_string = "{\"val\":\"Butter\"}";
    let v: FrenchToast = serde_json::from_str(serialized_string).unwrap();
    assert_eq!("FrenchToast { ingredient: \"Butter\" }", format!("{:?}", v))
}
