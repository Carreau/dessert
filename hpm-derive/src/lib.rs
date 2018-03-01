extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::Meta::{List};

use proc_macro::TokenStream;


fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = ast.ident;

    //
    let mut qname = "".to_owned();
    let mut tname = "".to_owned();

    println!("ATTRS:: {:?}", &ast.attrs);
    for attr in ast.attrs.iter() {
        if let Some(List(list)) = attr.interpret_meta() {
            for tpl in list.nested.iter(){
                match tpl {
                   &syn::NestedMeta::Meta(syn::Meta::Word(ref ins)) => {tname = ins.to_string();},
                   _ => ()
                };
            }
        }

        println!("Path {:?}", attr.path);

        let &syn::Path{ref segments, ..} = &attr.path;
        {
            println!("Looping segments {:?}", segments);
            for tpl in segments.iter(){
                let &syn::PathSegment{ref ident, ..} = tpl;
                println!("Testing ident {:?}", ident );
                if ident.to_string() == "via"{
                    println!("Moving Qname");
                    qname = tname.to_owned();
                }
            }
        }



    }



    println!("Qname is {}", &qname);
    quote! {
        impl HPM for #name {
            fn hpm() {
                println!("Hello, World ({})! My name is {}", stringify!(#qname), stringify!(#name));
            }
        }
    }
}

#[proc_macro_derive(HPM, attributes(via))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);
    
    // Return the generated impl
    gen.into()
}
