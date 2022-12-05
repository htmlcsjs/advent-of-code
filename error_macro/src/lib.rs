use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ErrorWrapper)]
pub fn error_wrapper(tokens: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input: DeriveInput = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;
    let mut impls = Vec::new();
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let variants = match input.data {
        Data::Enum(e) => e.variants,
        _ => panic!("This macro is for enum use only"),
    };

    for var in variants {
        let ident = var.ident;
        match var.fields {
            Fields::Named(data) => {
                if data.named.len() == 1 {
                    let impl_type = &data.named.first().expect("what the fuck").ty;
                    let field_name = data.named.first().expect("what the fuck").ident.as_ref().expect("wtf");
                    impls.push(quote! {
                        impl #impl_generics From<#impl_type> for #name #ty_generics #where_clause {
                            fn from(t: #impl_type) -> #name {
                                #name::#ident{#field_name: t}
                            }
                        }
                    });
                }
            }
            Fields::Unnamed(data) => {
                if data.unnamed.len() == 1 {
                    let impl_type = &data.unnamed.first().expect("what the fuck").ty;
                    impls.push(quote! {
                        impl #impl_generics From<#impl_type> for #name #ty_generics #where_clause {
                            fn from(t: #impl_type) -> #name {
                                #name::#ident(t)
                            }
                        }
                    });
                }
            }
            Fields::Unit => continue,
        }
    }
    TokenStream::from(quote! {#(#impls)*})
}
