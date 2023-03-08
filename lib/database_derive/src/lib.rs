extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Database, attributes(index, id))]
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
    let first_id = id.first().unwrap();

    let gen = quote! {
        use database::traits::Entity;
        use database::traits::Index;
        use database::traits::IndexType;

        impl Entity for #name {
            fn get_id(&self) -> u64 { self.#first_id }
            fn set_id(&mut self, id: u64) { self.#first_id = id; }
        }

        impl Index for #name {
            fn get_indexes(&self) -> Vec<(&str, IndexType)> {
                vec![#(#index_traits)*]
            }
        }

    };
    gen.into()
}

