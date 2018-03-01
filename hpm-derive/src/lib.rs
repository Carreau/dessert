extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::Meta::{List};

use proc_macro::TokenStream;



fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = ast.ident;

    //
    let mut qname: syn::Ident = syn::Ident::from("nothing");
    let mut tname: syn::Ident = syn::Ident::from("temp");

    println!("ATTRS:: {:?}", &ast.attrs);
    for attr in ast.attrs.iter() {
        if let Some(List(list)) = attr.interpret_meta() {
            for tpl in list.nested.iter(){
                match tpl {
                   &syn::NestedMeta::Meta(syn::Meta::Word(ins)) => {tname = ins;},
                   _ => ()
                };
            }
        }

        //println!("Path {:?}", attr.path);

        let &syn::Path{ref segments, ..} = &attr.path;
        {
            //println!("Looping segments {:?}", segments);
            for tpl in segments.iter(){
                let &syn::PathSegment{ref ident, ..} = tpl;
                //println!("Testing ident {:?}", ident );
                if ident.to_string() == "via"{
                    //println!("Moving Qname");
                    qname = tname.to_owned();
                }
            }
        }



    }



    println!("Qname is {}", &qname);

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

    
    let generated = quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _WAHT: () = {
            extern crate serde as _serde;
            #impl_block_2
            #impl_block
        };
    };

    println!("{:?}", generated);
    generated
}

#[proc_macro_derive(ViaDeserialize, attributes(via))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);
    
    // Return the generated impl
    gen.into()
}
