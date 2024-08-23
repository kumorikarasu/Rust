extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Database, attributes(index, id, timestamp, creation_timestamp))]
pub fn db_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_db(&ast)
}


fn impl_db(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let struct_ = match &ast.data {
        syn::Data::Struct(s) => s,
        _ => panic!("Only structs can be annotated with #[Database]")
    };

    // Get Indexes
    let index_traits = struct_.fields.iter().filter(|f| {
        f.attrs.iter().any(|a| {
            a.path.is_ident("index")
        })
    }).map(|f| {
        let ident = match &f.ident {
            Some(i) => i,
            None => panic!("Unnamed fields cannot be indexed")
        };
        let ty = match &f.ty {
            syn::Type::Path(p) => p,
            _ => panic!("Only simple types can be indexed")
        };
        let typ = ty.path.segments.last().unwrap();
        let ide = ident.to_string();

        quote! {
            (#ide, IndexType::#typ(self.#ident.clone()))
        }
    });

    // Get ID
    let id = struct_.fields.iter().filter(|f| {
        f.attrs.iter().any(|a| {
            a.path.is_ident("id")
        })
    }).map(|f| {
        let ident = match &f.ident {
            Some(i) => i,
            None => panic!("Unnamed fields cannot be indexed")
        };
        ident
    }).collect::<Vec<_>>();

    if id.len() != 1 {
        panic!("Only one field can be annotated with #[id]")
    }
    let first_id = id.first().unwrap();

    // Get Timestamp
    let timestamp = struct_.fields.iter().filter(|f| {
        f.attrs.iter().any(|a| {
            a.path.is_ident("timestamp")
        })
    }).map(|f| {
        let ident = match &f.ident {
            Some(i) => i,
            None => panic!("Unnamed fields cannot be indexed")
        };
        ident
    }).collect::<Vec<_>>();
    if timestamp.len() != 1 {
        panic!("Only one field can be annotated with #[timestamp]")
    }
    let timestamp = timestamp.first().unwrap();

    // Get Creation Timestamp
    let creation_timestamp = struct_.fields.iter().filter(|f| {
        f.attrs.iter().any(|a| {
            a.path.is_ident("creation_timestamp")
        })
    }).map(|f| {
        let ident = match &f.ident {
            Some(i) => i,
            None => panic!("Unnamed fields cannot be indexed")
        };
        ident
    }).collect::<Vec<_>>();
    if creation_timestamp.len() != 1 {
        panic!("Only one field can be annotated with #[creation_timestamp]")
    }
    let creation_timestamp = creation_timestamp.first().unwrap();

    // Template itself
    let gen = quote! {
        use database::traits::Entity;
        use database::traits::Index;
        use database::traits::IndexType;
        use database::traits::Timestamp;

        impl Entity for #name {
            fn get_id(&self) -> u64 { self.#first_id }
            fn set_id(&mut self, id: u64) { self.#first_id = id; }
        }

        impl Index for #name {
            fn get_indexes(&self) -> Vec<(&str, IndexType)> {
                vec![#(#index_traits)*]
            }
        }

        impl Timestamp for #name {
            fn get_creation_timestamp(&self) -> DateTime<Utc> {
                self.#creation_timestamp
            }

            fn set_creation_timestamp(&mut self, timestamp: DateTime<Utc>) {
                self.#creation_timestamp = timestamp;
            }

            fn set_timestamp(&mut self, timestamp: DateTime<Utc>) {
                self.#timestamp = timestamp;
            }
        }

    };
    gen.into()
}

