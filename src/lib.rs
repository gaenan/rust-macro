use proc_macro::TokenStream;
use quote::quote;

// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);

    // get the ident
    let ident = input.ident;
    // get generics
    let generics = input.generics;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be used on enums"),
    };
    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(_fields) => quote! {}, //fields.named.iter().collect::<Vec<_>>(),
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let fields = fields.unnamed.first().expect("should have one field");
                    let ty = &fields.ty;
                    // let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
    .into()
}
