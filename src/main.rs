#[macro_use]
extern crate hpm_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde::Deserialize;

trait ViaDeserialize {
    //fn hpm() -> ();
}

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
    let f = FrenchToast { a: 1 };
    //let b:B = f.into();
    let v: FrenchToast = serde_json::from_str("{\"val\":1}").unwrap();
    println!("{:?}", v)
    //Waffles::hpm();
}
