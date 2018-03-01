#[macro_use]
extern crate hpm_derive;
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate serde_derive;

use serde::{Deserialize};

trait HPM {
    //fn hpm() -> ();
}

#[derive(Deserialize)]
struct B {
    val: usize
}

#[derive(HPM, Debug)]
#[via(B)]
struct FrenchToast{
    a: usize
}


impl Into<B> for FrenchToast{

    fn into( self:Self) -> B {
        B{val:self.a}
    }
}

fn main() {
    let f = FrenchToast{a:1};
    //let b:B = f.into();
    let v:FrenchToast = serde_json::from_str("{\"val\":1}").unwrap();
    println!("{:?}", v)
    //Waffles::hpm();
}
