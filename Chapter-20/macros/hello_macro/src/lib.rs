use proc_macro::TokenStream;
use quote::quote;

// We are now inside the macro crate.
// The macro will be invoked as `HelloMacro`.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

    // Parse the input tokens into a syntax tree.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

// Generate the implementation of the HelloMacro trait.
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // Get the name of the struct or enum.
    let name = &ast.ident;
    // Generate the implementation.
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}