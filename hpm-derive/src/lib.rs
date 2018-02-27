extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;


use proc_macro::TokenStream;


fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = ast.ident;

    let opt_attr = ast.attrs.get(0);

    let it = match opt_attr {
        Some(ref attr) => Some(attr.interpret_meta()),
        None => None

    };

    println!("IM: {:?}", it);
    

    //println!(">> {:?}", opt_attr.unwrap().tts);

    //match opt_attr {
    //    &Some(attr) => println!("{}", attr),
    //    &None => (),
    //};

    quote! {
        impl HPM for #name {
            fn hpm() {
                println!("Hello, World! My name is {}", stringify!(#name));
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
