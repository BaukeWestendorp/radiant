use quote::quote;
use syn::parse::Parser;
use syn::{Data, DeriveInput, Generics, Ident, parse_macro_input};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let impls = gen_impls(&input.ident, &input.generics);

    let extracted = quote! {
        #impls
    };

    extracted.into()
}

fn gen_impls(ident: &Ident, generics: &Generics) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics crate::show::Object for #ident #ty_generics #where_clause {
            fn create(id: crate::show::ObjectId, pool_id: crate::show::PoolId, name: String) -> Self {
                Self { name, id, pool_id , ..Default::default() }
            }

            fn id(&self) -> crate::show::ObjectId {
                self.id
            }

            fn name(&self) -> &str {
                &self.name
            }

            fn set_name(&mut self, name: String) {
                self.name = name;
            }

            fn pool_id(&self) -> crate::show::PoolId {
                self.pool_id
            }

            fn set_pool_id(&mut self, pool_id: crate::show::PoolId) {
                self.pool_id = pool_id;
            }

            fn kind(&self) -> crate::show::ObjectKind {
                crate::show::ObjectKind::#ident
            }

            fn as_any(&self) -> &dyn std::any::Any { self }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

            fn into_any_object(self) -> crate::show::AnyObject {
                crate::show::AnyObject::from(self)
            }
        }

        impl #impl_generics From<#ident #ty_generics> for crate::show::AnyObject #where_clause {
            fn from(obj: #ident #ty_generics) -> crate::show::AnyObject {
                crate::show::AnyObject::#ident(obj)
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
                            .parse2(quote! { pub pool_id: crate::show::PoolId })
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
