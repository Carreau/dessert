//! # Dessert
//! 
//! A package to simplify custom SerDe Serialize/Deserialise traits. Instead of defining the traits
//! Manually, define an intermediate struct/enum as well as the From and/or Into traits necessary. 
//! Desert will take care of generating the Serialize/Deserialize traits for you.
//!
//! For example, de-serialising by renaming a field:
//!
//! ```rust, ignore
//! // proc_macro can't be tested in documentation  comments.
//! use dessert::{ViaDeserialize};
//!  
//! #[derive(ViaDeserialize, Debug)]
//! #[via(Intermediate)]
//! struct FrenchToast {
//!     ingredient: String,
//! }
//!  
//! #[derive(Deserialize)]
//! struct Intermediate {
//!     val: String,
//! }
//!  
//!  
//! impl From<Intermediate> for FrenchToast {
//!     fn from(b: Intermediate) -> Self {
//!         Self { ingredient: b.val }
//!     }
//! } 
//! 
//! let serialized_string= "{\"val\":\"Butter\"}";
//! let v: FrenchToast = serde_json::from_str(serialized_string).unwrap();
//! assert_eq!("FrenchToast { ingredient: \"Butter\" }", format!("{:?}",v))
//! ```
//!
//! You can use `cargo run --example deserialise` for a full working examples

extern crate serde;

/// use `#[derive(ViaDeserialize)]` to automatically implement SerDe's Deserialise traits via an
/// intermediate datasctructure. Your struct must implement `From<Intermediate>`.
pub trait ViaDeserialize {}


/// use `#[derive(ViaSerialize)]` to automatically implement SerDe's Serialise traits via an
/// intermediate datasctructure. Your struct must implement `Into<Intermediate>` and `Clone`.
pub trait ViaSerialize {}
