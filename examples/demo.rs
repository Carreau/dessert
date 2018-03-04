#[macro_use]
extern crate dessert_derive;
extern crate dessert;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use dessert::{ViaDeserialize, ViaSerialize};


#[derive(ViaDeserialize, ViaSerialize, Debug, Clone)]
#[via(Intermediate)]
struct FrenchToast {
    ingredient: String,
}

#[derive(Deserialize, Serialize)]
struct Intermediate {
    val: String,
}

impl From<Intermediate> for FrenchToast {
    fn from(b: Intermediate) -> Self {
        Self { ingredient: b.val }
    }
}

impl Into<Intermediate> for FrenchToast {
    fn into(self) -> Intermediate {
        Intermediate{ val: self.ingredient}
    }
}


// impl From<FrenchToast> for Intermediate {
//      fn from(ft:FrenchToast) -> Self {
//          Self{val: ft.ingredient}
//      }
// }

#[test]
fn test_main() -> (){
    main()
}

fn main() -> () {
    let serialized_string= "{\"val\":\"Butter\"}";
    let v: FrenchToast = serde_json::from_str(serialized_string).unwrap();
    println!("Serialized form: {}", serde_json::to_string(&v).unwrap());
    println!("Debug format : {:?}", v)

}

