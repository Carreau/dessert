//! # Dessert-Derive
//! 
//! Provide derive macros for the `desert` crate, which provide a simpler interface to implement
//! custom SerDe `Serialize` and `Deserialize` traits.
//!

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::Meta::{List};

use proc_macro::TokenStream;



fn impl_viadmacro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = ast.ident;

    let mut qname: syn::Ident = syn::Ident::from("nothing");
    let mut tname: syn::Ident = syn::Ident::from("temp");

    for attr in ast.attrs.iter() {
        if let Some(List(list)) = attr.interpret_meta() {
            for tpl in list.nested.iter(){
                match tpl {
                   &syn::NestedMeta::Meta(syn::Meta::Word(ins)) => {tname = ins;},
                   _ => ()
                };
            }
        }


        let &syn::Path{ref segments, ..} = &attr.path;
        {
            for tpl in segments.iter(){
                let &syn::PathSegment{ref ident, ..} = tpl;
                if ident.to_string() == "via"{
                    qname = tname.to_owned();
                }
            }
        }



    }

    let impl_block = quote! {
            impl ViaDeserialize for #name { };
    };


    let impl_block_2 = quote! {
            impl<'de> serde::Deserialize<'de> for #name
            //where
            //    #qname: Into<#name>,
            //    #qname: _serde::Deserialize<'de>,
            {
                 fn deserialize<D>(deserializer: D) -> Result<#name, D::Error>
                 where
                    D: _serde::Deserializer<'de>
                 {
                    match #qname::deserialize(deserializer) {
                        Ok(x) => Ok(#name::from(x)),
                        Err(r) => Err(r),
                    }
                 }
            };
            
    };

    
    quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _WAHT: () = {
            extern crate serde as _serde;
            #impl_block_2
            #impl_block
        };
    }

}


fn impl_viasmacro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = ast.ident;

    let mut qname: syn::Ident = syn::Ident::from("nothing");
    let mut tname: syn::Ident = syn::Ident::from("temp");

    for attr in ast.attrs.iter() {
        if let Some(List(list)) = attr.interpret_meta() {
            for tpl in list.nested.iter(){
                match tpl {
                   &syn::NestedMeta::Meta(syn::Meta::Word(ins)) => {tname = ins;},
                   _ => ()
                };
            }
        }


        let &syn::Path{ref segments, ..} = &attr.path;
        {
            for tpl in segments.iter(){
                let &syn::PathSegment{ref ident, ..} = tpl;
                if ident.to_string() == "via"{
                    qname = tname.to_owned();
                }
            }
        }



    }

    let impl_block = quote! {
            impl ViaSerialize for #name { };
    };


    let where_clause = quote!{
        where #qname: _serde::Serialize,
              #name: Into<#qname>,
              #name: Clone
    };


    // quote does not seem to like where clauses for whaever reason.
    //println!("WHERE CLAUSE {:?} ", where_clause);

    let impl_block_2 = quote! {

        impl _serde::Serialize for #name
        #where_clause
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: _serde::Serializer,
            {
                let n:#name = self.clone();
                let dn:#qname = n.into();
                dn.serialize(serializer)
            }
        }


            
    };

    
    quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _SERIALISE: () = {
            extern crate serde as _serde;
            #impl_block_2
            #impl_block
        };
    }

}

/// This function is responsible for taking a TokenStream 
/// and generate the appropriate code to derive `ViaDeserialize`.
/// use the `#[derive(ViaDeserialize)]`
#[proc_macro_derive(ViaDeserialize, attributes(via))]
pub fn viad_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_viadmacro(&ast);
    gen.into()
}


/// This function is responsible for taking a TokenStream 
/// and generate the appropriate code to derive `ViaSerialize`.
/// use the `#[derive(ViaSerialize)]`
#[proc_macro_derive(ViaSerialize, attributes(via))]
pub fn vias_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_viasmacro(&ast);
    gen.into()
}
