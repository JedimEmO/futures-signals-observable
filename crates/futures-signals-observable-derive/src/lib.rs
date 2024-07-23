use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Fields;

#[proc_macro_derive(Observable)]
pub fn derive_observable(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Ok(str) = syn::parse::<syn::ItemStruct>(tokens.clone()) {
        let struct_ident = str.ident.clone();

        let member_changes = str.fields.iter().map(|field| {
            let field_ident = field.ident.as_ref().expect("only named fields supported");

            quote! { self. #field_ident . changed().boxed(),}
        });

        quote! {
            impl futures_signals_observable::Observable for #struct_ident {
                fn changed(&self) -> impl futures::Stream<Item=()>  + Send + 'static {
                    futures::stream::select_all([
                        #(#member_changes)*
                    ])
                }
            }
        }
        .into()
    } else if let Ok(enu) = syn::parse::<syn::ItemEnum>(tokens.clone()) {
        let enum_ident = enu.ident.clone();

        let variant_changes = enu.variants.iter().map(|variant| {
            let variant_ident = variant.ident.clone();

            match &variant.fields {
                Fields::Unit => quote! {
                    Self:: #variant_ident => futures::stream::iter([]).boxed(),
                },
                Fields::Named(named) => {
                    let named_idents = named.named.iter().map(|n| n.ident.clone().unwrap());
                    let named_watches = named.named.iter().map(|n| {
                        let ident = n.ident.clone().unwrap();

                        quote! { #ident .changed().boxed() }
                    });

                    quote! {
                        Self:: #variant_ident { #(#named_idents),* } => {
                            futures::stream::select_all([
                                #(#named_watches),*
                            ]).boxed()
                        }
                    }
                }
                Fields::Unnamed(unnamed) => {
                    let unnamed_idents = (0..unnamed.unnamed.len()).map(|i| {
                        let ident = format!("v_{}", i);
                        syn::Ident::new(ident.as_str(), proc_macro2::Span::call_site())
                    });

                    let unnamed_watches = unnamed.unnamed.iter().enumerate().map(|(i, n)| {
                        let ident = format!("v_{}", i);
                        let ident = syn::Ident::new(ident.as_str(), proc_macro2::Span::call_site());

                        quote! { #ident.changed().boxed() }
                    });

                    quote! {
                        Self:: #variant_ident ( #(#unnamed_idents),* ) => {
                            futures::stream::select_all([
                                #(#unnamed_watches),*
                            ]).boxed()
                        }
                    }
                }
            }
        });

        quote! {
            impl futures_signals_observable::Observable for #enum_ident {
                fn changed(&self) -> impl futures::Stream<Item=()>  + Send + 'static {
                    match self {
                        #(#variant_changes)*
                    }
                }
            }
        }
        .into()
    } else {
        let span = Span::call_site();
        syn::Error::new(span, "expected struct or enum")
            .to_compile_error()
            .into()
    }
}
