use proc_macro::TokenStream;
use quote::quote;

fn get_regex_attr(ast: &syn::DeriveInput) -> String {
    for attr in &ast.attrs {
        if attr.path.is_ident("regex") {
            let lit: syn::LitStr = attr.parse_args().expect("The regex must be a string");
            let regex = lit.value();
            return regex;
        }
    }

    panic!("The regex attribute must be specified");
}

fn get_fields(ast: &syn::DeriveInput) -> Vec<syn::Ident> {
    let mut fields = Vec::new();

    match &ast.data {
        syn::Data::Struct(data_struct) => {
            match &data_struct.fields {
                syn::Fields::Named(named) => {
                    for field in &named.named {
                        fields.push(field.ident.clone().unwrap());
                    }
                }

                _ => panic!("Only structs with named fields are supported")
            }
        }

        _=> panic!("Only structs are supported")
    }

    fields
}

#[proc_macro_derive(FromStr, attributes(regex))]
pub fn fromstr_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let regex = get_regex_attr(&ast);
    let fields = get_fields(&ast);

    let gen = quote! {
        impl ::std::str::FromStr for #name {
            type Err = ::anyhow::Error;

            fn from_str(s: &str) -> ::anyhow::Result<#name> {
                let re = ::regex::Regex::new(#regex).unwrap();
                let caps = re.captures(s).ok_or_else(|| ::anyhow::anyhow!("Invalid format"))?;

                Ok(#name {
                    #(#fields: caps.name(stringify!(#fields)).unwrap().as_str().parse().unwrap()),*
                })
            }
        }
    };

    gen.into()
}
