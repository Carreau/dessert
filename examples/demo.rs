#[macro_use]
extern crate dessert_derive;
extern crate dessert;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use dessert::ViaDeserialize;

#[derive(Deserialize)]
struct B {
    val: usize,
}

#[derive(ViaDeserialize, Debug)]
#[via(B)]
struct FrenchToast {
    a: usize,
}

impl From<B> for FrenchToast {
    fn from(b: B) -> Self {
        Self { a: b.val }
    }
}

fn main() {
    let v: FrenchToast = serde_json::from_str("{\"val\":1}").unwrap();
    println!("{:?}", v)
}
