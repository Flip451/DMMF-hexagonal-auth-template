use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[derive(FromDeriveInput)]
#[darling(attributes(entity))]
struct EntityArgs {
    ident: syn::Ident,
    data: darling::ast::Data<EntityVariantArgs, EntityFieldArgs>,
    #[darling(default)]
    domain_path: Option<String>,
}

#[derive(FromField)]
#[darling(attributes(entity))]
struct EntityFieldArgs {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    id: bool,
}

#[derive(FromVariant)]
struct EntityVariantArgs {
    ident: syn::Ident,
    fields: darling::ast::Fields<EntityFieldArgs>,
}

#[proc_macro_derive(Entity, attributes(entity))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let args = match EntityArgs::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let name = &args.ident;

    // Determine the crate path
    let domain_path = if let Some(path_str) = args.domain_path {
        let p: syn::Path = syn::parse_str(&path_str).expect("Invalid domain_path");
        quote!(#p)
    } else {
        let pkg_name = std::env::var("CARGO_PKG_NAME").ok();
        if pkg_name.as_deref() == Some("domain") {
            quote!(crate)
        } else {
            quote!(::domain)
        }
    };

    match &args.data {
        darling::ast::Data::Struct(fields) => {
            let id_field = fields.iter().find(|f| f.id);
            if let Some(id_field) = id_field {
                let id_ident = id_field
                    .ident
                    .as_ref()
                    .expect("Entity field must have an identifier");
                let id_type = &id_field.ty;

                let expanded = quote! {
                    impl #domain_path::Entity for #name {
                        type Id = #id_type;
                        fn identity(&self) -> &#id_type {
                            &self.#id_ident
                        }
                    }

                    impl PartialEq for #name {
                        fn eq(&self, other: &Self) -> bool {
                            use #domain_path::Entity;
                            self.identity() == other.identity()
                        }
                    }

                    impl Eq for #name {}
                };
                TokenStream::from(expanded)
            } else {
                TokenStream::from(
                    darling::Error::custom("Entity must have a field marked with #[entity(id)]")
                        .write_errors(),
                )
            }
        }
        darling::ast::Data::Enum(variants) => {
            let mut match_arms = Vec::new();
            let mut first_field_type = None;

            for variant in variants {
                if variant.fields.len() != 1 {
                    return TokenStream::from(
                        darling::Error::custom(
                            "Enum variants for Entity must have exactly one field",
                        )
                        .with_span(&variant.ident)
                        .write_errors(),
                    );
                }
                let v_ident = &variant.ident;
                let field = &variant.fields.fields[0];
                let field_type = &field.ty;

                if first_field_type.is_none() {
                    first_field_type = Some(field_type.clone());
                }

                match_arms.push(quote! {
                    Self::#v_ident(inner) => inner.identity(),
                });
            }

            if let Some(field_type) = first_field_type {
                let expanded = quote! {
                    impl #domain_path::Entity for #name {
                        type Id = <#field_type as #domain_path::Entity>::Id;

                        fn identity(&self) -> &Self::Id {
                            match self {
                                #(#match_arms)*
                            }
                        }
                    }

                    impl PartialEq for #name {
                        fn eq(&self, other: &Self) -> bool {
                            use #domain_path::Entity;
                            self.identity() == other.identity()
                        }
                    }

                    impl Eq for #name {}
                };
                TokenStream::from(expanded)
            } else {
                TokenStream::from(
                    darling::Error::custom("Enum must have at least one variant").write_errors(),
                )
            }
        }
    }
}

#[proc_macro_derive(SensitiveDebug)]
pub fn derive_sensitive_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let pkg_name = std::env::var("CARGO_PKG_NAME").ok();
    let domain_path = if pkg_name.as_deref() == Some("domain") {
        quote!(crate)
    } else {
        quote!(::domain)
    };

    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use #domain_path::SensitiveData;
                write!(f, "\"{}\"", self.to_masked_string())
            }
        }
    };

    TokenStream::from(expanded)
}
