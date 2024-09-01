#![doc = include_str!("../README.md")]

use std::ffi::CString;

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Literal};
use quote::ToTokens;
use syn::{parse_macro_input, spanned::Spanned, Error, ItemEnum, parse_quote, Lit};

/// auto implement the TryFrom<Literal> trait and Into<Literal> trait
/// where the `literal` must be only one type
#[proc_macro_derive(LiteralEnum, attributes(lit))]
pub fn easy(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);

    match real_easy(item) {
        Ok(v) => v.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn real_easy(item: ItemEnum) -> Result<TokenStream2, Error> {
    use std::collections::HashMap;

    if !item.generics.to_token_stream().is_empty() {
        return Err(Error::new(item.generics.span(), "generics is forbidden"));
    }

    let mut first_lit: Option<syn::Lit> = None;
    let mut ty: Option<syn::Type> = None;
    let mut lit_value = Vec::new();
    let mut var_ident = Vec::new();

    let mut lit_hashmap = HashMap::<String, syn::Lit>::new(); // avoid duplicate literals

    for var in item.variants.iter() {
        if !matches!(var.fields, syn::Fields::Unit) {
            return Err(Error::new(
                var.span(),
                "every variant must be Unit kind, like `None`",
            ));
        }

        let attr = match var.attrs.iter().find(|attr| attr.path().is_ident("lit")) {
            Some(attr) => attr.clone(),
            None => {
                let lit = if let Some(ref lit) = first_lit {
                    match lit {
                        Lit::Str(_) => Lit::new(Literal::string(&var.ident.to_string())),
                        Lit::ByteStr(_) => Lit::new(Literal::byte_string(var.ident.to_string().as_bytes())),
                        Lit::CStr(_) => Lit::new(Literal::c_string(CString::new(var.ident.to_string()).unwrap().as_c_str())),
                        Lit::Byte(_) => {
                            let ident = var.ident.to_string();
                            if ident.len() != 1 {
                                return Err(Error::new(var.ident.span(), "Unable to convert this value to a Byte"));
                            }
                            Lit::new(Literal::byte_character(ident.as_bytes()[0]))
                        },
                        Lit::Char(_) => {
                            let ident = var.ident.to_string();
                            if ident.len() != 1 {
                                return Err(Error::new(var.ident.span(), "Unable to convert this value to a Char"));
                            }
                            Lit::new(Literal::character(ident.chars().next().unwrap()))
                        },
                        // Lit::Int(_) => todo!(),
                        // Lit::Bool(_) => todo!(),
                        // Lit::Float(_) => todo!(),
                        // Lit::Verbatim(_) => todo!(),
                        _ => return Err(Error::new(var.ident.span(), "Unable to infer the type of this value")),
                    }
                } else {
                    Lit::new(Literal::string(&var.ident.to_string()))
                };

                // eprintln!("lit: {}", lit.to_token_stream().to_string());

                parse_quote! {
                    #[lit = #lit]
                }
            }
        };

        let syn::Meta::NameValue(ref name_value) = attr.meta else {
            return Err(Error::new(attr.meta.span(), "the format should be like: `#[lit = 42]`"));
        };

        let syn::Expr::Lit(syn::ExprLit{ref lit, ..}) = name_value.value else {
            return Err(Error::new(name_value.span(), "the value should be a literal"));
        };

        if first_lit.is_none() {
            first_lit = Some(lit.clone());
        }

        if let Some(ref t) = ty {
            if t.to_token_stream().to_string() != lit_to_ty(lit)?.to_token_stream().to_string() {
                return Err(Error::new(
                    lit.span(),
                    "All the literals must be the same type",
                ));
            }
        } else {
            ty = Some(lit_to_ty(lit)?);
        }

        var_ident.push(var.ident.clone());

        let lit_str = lit.to_token_stream().to_string();
        if let Some(it) = lit_hashmap.get(&lit_str) {
            let mut err = Error::new(lit.span(), format!("{} is declared twice", lit_str));
            err.combine(Error::new(
                it.span(),
                format!("{} is declared here first", lit_str),
            ));
            return Err(err);
        }
        lit_hashmap.insert(lit_str, lit.clone());
        lit_value.push(lit.clone());
    }

    let enum_ident = item.ident;

    match ty {
        Some(lit_ty) => Ok(derive(enum_ident, var_ident, lit_ty, lit_value)),
        None => Ok(TokenStream2::new()),
    }
}

fn derive(
    enum_ident: syn::Ident,
    var_ident: Vec<syn::Ident>,
    lit_ty: syn::Type,
    lit_value: Vec<syn::Lit>,
) -> TokenStream2 {
    let life = if let syn::Type::Reference(ref type_reference) = lit_ty {
        &type_reference.lifetime
    } else {
        &None
    };

    quote::quote! {
        impl<#life> TryFrom<#lit_ty> for #enum_ident {
            type Error = #lit_ty;

            fn try_from(value: #lit_ty) -> Result<Self, Self::Error> {
                // // error: cannot use unsized non-slice type `CStr` in constant patterns
                // match value {
                //     #(#lit_value => Ok(Self::#var_ident),)*
                //     _ => Err(value),
                // }

                #(if #lit_value == value {
                    return Ok(Self::#var_ident);
                })*
                Err(value)
            }
        }

        impl<#life> Into<#lit_ty> for #enum_ident {
            fn into(self) -> #lit_ty {
                match self {
                    #(Self::#var_ident => #lit_value,)*
                }
            }
        }
    }
}

fn lit_to_ty(lit: &syn::Lit) -> Result<syn::Type, Error> {
    let ty = match lit {
        Lit::Str(_) => syn::parse_str("&'a str").unwrap(),
        Lit::ByteStr(_) => syn::parse_str("&'a [u8]").unwrap(),
        Lit::CStr(_) => syn::parse_str("&'a std::ffi::CStr").unwrap(),
        Lit::Byte(_) => syn::parse_str("u8").unwrap(),
        Lit::Char(_) => syn::parse_str("char").unwrap(),
        Lit::Int(int) => {
            if int.suffix().is_empty() {
                syn::parse_str("u32").unwrap()
            } else {
                syn::parse_str(int.suffix()).unwrap()
            }
        }

        Lit::Bool(_) => syn::parse_str("bool").unwrap(),
        // syn::Lit::Float(_) => syn::parse_str("f64").unwrap(), // floating-point types cannot be used in patterns
        // syn::Lit::Verbatim(_) => syn::parse_str("&'static str").unwrap(),
        _ => return Err(Error::new(lit.span(), "This type is not supported")),
    };

    Ok(ty)
}
