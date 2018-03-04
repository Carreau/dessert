//! # Dessert
//!
//! A package to simplify custom SerDe Serialize/Deserialise traits. Instead of defining the traits
//! Manually, define an intermediate struct/enum as well as the From and/or Into traits necessary.
//! Desert will take care of generating the Serialize/Deserialize traits for you.
//!
//! ## De-Serialise
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
//!
//! ## Serialise
//!
//! The struct you want to serialise must implement `Clone` (looking at lifting this restriction)
//! and should implement `Into<Intermediate>`. `Intermediate` must implement serde's `Serialize`.
//!
//! Use `#[derive(ViaSerialize)]` and
//! `#[via(Intermediate)]` to automatically derive the serde `Serialise` trait.
//!
//! ```rust, ignore
//! // proc_macro can't be tested in documentation  comments.
//! use dessert::ViaSerialize;
//! 
//! #[derive(ViaSerialize, Clone,  Debug)]
//! #[via(Intermediate)]
//! struct FrenchToast {
//!     ingredient: String,
//! }
//! 
//! #[derive(Serialize)]
//! struct Intermediate {
//!     val: String,
//! }
//! 
//! impl Into<Intermediate> for FrenchToast {
//!     fn into(self) -> Intermediate {
//!         Intermediate { val: self.ingredient }
//!     }
//! }
//! 
//! let v: FrenchToast =  FrenchToast{ingredient:"Butter".to_owned()};
//! let ser = serde_json::to_string(&v).unwrap();
//! assert_eq!(ser, "{\"val\":\"Butter\"}")
//! ```
//!
//! Try `cargo run --examples serialise`

extern crate serde;

/// use `#[derive(ViaDeserialize)]` to automatically implement SerDe's Deserialise traits via an
/// intermediate datasctructure. Your struct must implement `From<Intermediate>`.
pub trait ViaDeserialize {}

/// use `#[derive(ViaSerialize)]` to automatically implement SerDe's Serialise traits via an
/// intermediate datasctructure. Your struct must implement `Into<Intermediate>` and `Clone`.
pub trait ViaSerialize {}
