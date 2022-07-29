extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{ItemTrait};
#[proc_macro_attribute]
pub fn polymorphic(_attr_data: TokenStream, original_item: TokenStream) -> TokenStream {

    let item = syn::parse_str::<ItemTrait>(&original_item.to_string());
    let item = match item {
        Ok(d) => d,
        Err(_) => return original_item ,
    };
    let name = format_ident!("{}",&item.ident.to_string());

    let expanded = quote!(
        #item
        impl<T: #name> CanBe<dyn #name> for CheckCasting<T, dyn #name> {}
    );
    TokenStream::from(expanded)
}
