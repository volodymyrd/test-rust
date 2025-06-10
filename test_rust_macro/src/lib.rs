use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(MatcherBase)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        ident, generics, ..
    } = ast;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics MatcherBase for #ident #ty_generics #where_clause {}
    }
    .into()
}
