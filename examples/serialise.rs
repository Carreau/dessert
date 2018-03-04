extern crate dessert;
#[macro_use]
extern crate dessert_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use dessert::ViaSerialize;

#[derive(ViaSerialize, Clone, Debug)]
#[via(Intermediate)]
struct FrenchToast {
    ingredient: String,
}

#[derive(Serialize)]
struct Intermediate {
    val: String,
}

impl Into<Intermediate> for FrenchToast {
    fn into(self) -> Intermediate {
        Intermediate {
            val: self.ingredient,
        }
    }
}

#[test]
fn test_main() -> () {
    main()
}

fn main() -> () {
    let v: FrenchToast = FrenchToast {
        ingredient: "Butter".to_owned(),
    };
    let ser = serde_json::to_string(&v).unwrap();
    assert_eq!(ser, "{\"val\":\"Butter\"}")
}
