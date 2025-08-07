mod deserialize;
mod serialize;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{Attribute, DeriveInput, Expr, Field, Lit, LitInt, Meta, Token, punctuated::Punctuated};

#[proc_macro_derive(
    Deserialize,
    attributes(padding, pre_disc_padding, post_disc_padding, trailing_padding)
)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    deserialize::derive(input)
}

#[proc_macro_derive(
    Serialize,
    attributes(padding, pre_disc_padding, post_disc_padding, trailing_padding)
)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    serialize::derive(input)
}

fn get_enum_type(input: &DeriveInput) -> Ident {
    for attr in &input.attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }
        let list = match &attr.meta {
            Meta::List(x) => x,
            _ => continue,
        };
        if list.tokens.is_empty() {
            panic!("encountered repr attribute with no arguments");
        }
        let nested = attr
            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .unwrap();
        for meta in nested {
            let path = match meta {
                Meta::Path(x) => x,
                _ => continue,
            };
            if path.is_ident("C") || path.is_ident("transparent") {
                continue;
            }
            return (*path.get_ident().expect("invalid repr attribute argument")).clone();
        }
    }
    panic!("You need to add a repr attribute to specify the discriminant type, e.g. #[repr(u16)]");
}

fn get_padding(attrs: &Vec<Attribute>, attr_name: &str) -> Option<LitInt> {
    for attr in attrs {
        if !attr.path().is_ident(attr_name) {
            continue;
        }
        let lit = match &attr.meta {
            Meta::NameValue(x) => match &x.value {
                Expr::Lit(expr_lit) => &expr_lit.lit,
                _ => panic!("{attr_name} value must be a literal"),
            },
            _ => panic!("{attr_name} needs to be name=value"),
        };
        let int_lit = match lit {
            Lit::Int(x) => x.clone(),
            _ => panic!("{attr_name} needs to be an integer"),
        };
        return Some(int_lit);
    }
    None
}

fn get_field_padding(input: &Field) -> Option<LitInt> {
    get_padding(&input.attrs, "padding")
}

fn get_pre_disc_padding(input: &DeriveInput) -> Option<LitInt> {
    get_padding(&input.attrs, "pre_disc_padding")
}

fn get_post_disc_padding(input: &DeriveInput) -> Option<LitInt> {
    get_padding(&input.attrs, "post_disc_padding")
}

fn get_trailing_padding(input: &DeriveInput) -> Option<LitInt> {
    get_padding(&input.attrs, "trailing_padding")
}
