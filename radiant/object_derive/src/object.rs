use quote::quote;
use syn::parse::Parser;
use syn::{Data, DeriveInput, Generics, Ident, parse_macro_input};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let impl_object = gen_impl_object(&input.ident, &input.generics);

    let extracted = quote! {
        #impl_object
    };

    extracted.into()
}

fn gen_impl_object(ident: &Ident, generics: &Generics) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics crate::show::Object for #ident #ty_generics #where_clause {
            fn create(id: crate::show::ObjectId, pool_id: crate::show::PoolId<Self>, name: String) -> Self
            {
                Self { name, id, pool_id , ..Default::default() }
            }

            fn name(&self) -> &str {
                &self.name
            }

            fn set_name(&mut self, name: String)
            {
                self.name = name;
            }

            fn id(&self) -> crate::show::ObjectId {
                self.id
            }

            fn pool_id(&self) -> crate::show::PoolId<Self> {
                self.pool_id
            }

            fn set_pool_id(&mut self, pool_id: crate::show::PoolId<Self>) {
                self.pool_id = pool_id;
            }
        }
    }
}

pub fn attribute(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut ast = { parse_macro_input!(input as DeriveInput) };

    match &mut ast.data {
        Data::Struct(struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! { pub name: String }).unwrap());
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { pub id: crate::show::ObjectId })
                            .unwrap(),
                    );
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { pub pool_id: crate::show::PoolId<Self> })
                            .unwrap(),
                    );
                }
                _ => (),
            }

            return quote! {
                #ast
            }
            .into();
        }
        _ => panic!("`object` has to be used on a struct"),
    }
}
