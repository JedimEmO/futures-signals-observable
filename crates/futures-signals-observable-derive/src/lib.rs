use proc_macro::Span;
use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_derive(Observable)]
pub fn derive_observable(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let str = syn::parse::<syn::ItemStruct>(tokens).unwrap();
    let struct_ident = str.ident.clone();

    let member_changes = str.fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("only named fields supported");

        quote! { self. #field_ident . changed().boxed(),}
    });

    quote! {
        impl futures_signals_observable::Observable for #struct_ident {
            fn changed(&self) -> impl futures::Stream<Item=()> {
                 futures::stream::select_all([
                    #(#member_changes)*
                ])
            }
        }
    }
    .into()
}
