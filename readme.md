# Desert 

The cherry on the cake (on top of [SerDe](https://serde.rs/)) to simplify custom `serialize` and `deserialize` traits. 

## Why

Writing visitor/deserialise and serialize traits can be quite annoying, and sometime you just know how to transform your struct(s) into another form that suits your need. 
With dessert you just need to create an intermediate struct and the From/Into traits to convert to it, and tell dessert to use this struct as an intermediate.

## example:

You can run examples with `$cargo run --examples demo`

```rust
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

// Serialized form: {"val":"Butter"}
// Debug format : FrenchToast { ingredient: "Butter" }
```
