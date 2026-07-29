use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident, Visibility, parse_macro_input,
};

#[proc_macro_derive(Input, attributes(rd_ui))]
pub fn derive_input(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    match &input.data {
        Data::Struct(data) => {
            return TokenStream::from(expand_struct_form_impl(name, data, &input.vis));
        }
        Data::Enum(data) => {
            let all_unit =
                data.variants.iter().all(|variant| matches!(variant.fields, Fields::Unit));

            if all_unit {
                return TokenStream::from(expand_picker_impl(name, data));
            } else {
                return TokenStream::from(expand_tagged_form_impl(name, data, &input.vis));
            }
        }
        Data::Union(_) => {
            return Error::new_spanned(name, "Input can only be derived for structs or enums")
                .to_compile_error()
                .into();
        }
    }
}

fn expand_picker_impl(name: &Ident, data: &DataEnum) -> proc_macro2::TokenStream {
    let mut variants_list = Vec::new();
    let mut match_arms = Vec::new();

    for variant in &data.variants {
        let var_ident = &variant.ident;
        let var_name_str = var_ident.to_string();

        variants_list.push(quote! {
            #name::#var_ident
        });

        match_arms.push(quote! {
            #name::#var_ident => #var_name_str.to_string()
        });
    }

    quote! {
        impl ::rd_ui::PickerValue for #name {
            fn variants() -> Vec<Self>
            where
                Self: Sized
            {
                vec![
                    #( #variants_list, )*
                ]
            }

            fn label(&self) -> String {
                match self {
                    #( #match_arms, )*
                }
            }
        }

        impl ::rd_ui::AutoInput for #name {
            type Delegate = ::rd_ui::Picker<Self>;

            fn build_input(
                initial_value: Self,
                window: &mut ::rd_ui::gpui::Window,
                cx: &mut ::rd_ui::gpui::App,
            ) -> ::rd_ui::gpui::Entity<::rd_ui::InputState<Self::Delegate>> {
                use ::rd_ui::gpui::AppContext as _;
                cx.new(|cx| {
                    ::rd_ui::InputState::new(
                        if <Self as ::rd_ui::PickerValue>::variants().len() > 5 {
                            ::rd_ui::Picker::dropdown(initial_value, cx.focus_handle(), window, cx)
                        } else {
                            ::rd_ui::Picker::inline(initial_value, cx.focus_handle(), window, cx)
                        },
                        window,
                        cx,
                    )
                })
            }
        }
    }
}

fn expand_struct_form_impl(
    name: &Ident,
    data: &DataStruct,
    vis: &Visibility,
) -> proc_macro2::TokenStream {
    let form_name = format_ident!("{}Form", name);

    let mut form_fields = Vec::new();
    let mut fields_push = Vec::new();
    let mut extract_fields = Vec::new();
    let mut field_builds = Vec::new();
    let mut form_struct_inits = Vec::new();

    for (f_idx, field) in data.fields.iter().enumerate() {
        let ty = &field.ty;

        let mut f_label =
            if let Some(ident) = &field.ident { ident.to_string() } else { f_idx.to_string() };

        for attr in &field.attrs {
            if attr.path().is_ident("rd_ui") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("label") {
                        let value = meta.value()?;
                        let lit: syn::LitStr = value.parse()?;
                        f_label = lit.value();
                    }
                    Ok(())
                });
            }
        }

        let form_field_ident = if let Some(ident) = &field.ident {
            ident.clone()
        } else {
            format_ident!("f{}", f_idx)
        };

        form_fields.push(quote! {
            #form_field_ident: ::rd_ui::gpui::Entity<::rd_ui::InputState<<#ty as ::rd_ui::AutoInput>::Delegate>>
        });

        let access = if let Some(ident) = &field.ident {
            quote! { initial_value.#ident }
        } else {
            let idx = syn::Index::from(f_idx);
            quote! { initial_value.#idx }
        };

        field_builds.push(quote! {
            let #form_field_ident = <#ty as ::rd_ui::AutoInput>::build_input(#access, window, cx);
        });

        form_struct_inits.push(quote! { #form_field_ident });

        fields_push.push(quote! {
            fields.push(::rd_ui::FormField::new(
                #f_label,
                ::rd_ui::Input::new(self.#form_field_ident.clone()),
                cx
            ));
        });

        let extract_val =
            quote! { self.#form_field_ident.read(cx).delegate().value_or_default(cx).clone() };
        if let Some(ident) = &field.ident {
            extract_fields.push(quote! { #ident: #extract_val });
        } else {
            extract_fields.push(quote! { #extract_val });
        }
    }

    let extract_expr = match &data.fields {
        Fields::Named(_) => quote! { #name { #( #extract_fields ),* } },
        Fields::Unnamed(_) => quote! { #name( #( #extract_fields ),* ) },
        Fields::Unit => quote! { #name },
    };

    quote! {
        #vis struct #form_name {
            #( #form_fields, )*
        }

        impl ::rd_ui::FormDelegate for #form_name {
            type Data = #name;

            fn fields(&self, cx: &mut ::rd_ui::gpui::App) -> Vec<::rd_ui::FormField> {
                let mut fields = Vec::new();
                #( #fields_push )*
                fields
            }

            fn extract_data(&self, cx: &::rd_ui::gpui::App) -> Option<Self::Data> {
                use ::rd_ui::InputDelegate as _;
                Some(#extract_expr)
            }
        }

        impl ::rd_ui::AutoInput for #name {
            type Delegate = ::rd_ui::Form<#form_name>;

            fn build_input(
                initial_value: Self,
                window: &mut ::rd_ui::gpui::Window,
                cx: &mut ::rd_ui::gpui::App,
            ) -> ::rd_ui::gpui::Entity<::rd_ui::InputState<Self::Delegate>> {
                use ::rd_ui::gpui::AppContext as _;

                #( #field_builds )*

                cx.new(move |cx| {
                    let delegate = #form_name {
                        #( #form_struct_inits, )*
                    };

                    let form = ::rd_ui::Form::new(delegate, cx.focus_handle(), window, cx);
                    ::rd_ui::InputState::new(form, window, cx)
                })
            }
        }
    }
}

fn expand_tagged_form_impl(
    name: &Ident,
    data: &DataEnum,
    vis: &Visibility,
) -> proc_macro2::TokenStream {
    let kind_name = format_ident!("{}Kind", name);
    let form_name = format_ident!("{}Form", name);

    let mut kind_variants = Vec::new();
    let mut from_arms = Vec::new();

    let mut form_fields = Vec::new();
    let mut fields_match_arms = Vec::new();
    let mut extract_match_arms = Vec::new();

    let mut init_extractions = Vec::new();
    let mut field_builds = Vec::new();
    let mut form_struct_inits = Vec::new();

    for (v_idx, variant) in data.variants.iter().enumerate() {
        let v_ident = &variant.ident;
        kind_variants.push(quote! { #v_ident });

        let is_unit = matches!(variant.fields, Fields::Unit);

        if is_unit {
            from_arms.push(quote! { #name::#v_ident => #kind_name::#v_ident });
            fields_match_arms.push(quote! { #kind_name::#v_ident => {} });
            extract_match_arms.push(quote! { #kind_name::#v_ident => Some(#name::#v_ident) });
            continue;
        }

        let mut original_field_binds = Vec::new();
        let mut field_tys = Vec::new();
        let mut field_labels = Vec::new();
        let mut form_field_idents = Vec::new();

        for (f_idx, field) in variant.fields.iter().enumerate() {
            let ty = &field.ty;

            let f_bind = if let Some(ident) = &field.ident {
                quote!(#ident)
            } else {
                let ident = format_ident!("f{}", f_idx);
                quote!(#ident)
            };

            let mut f_label =
                if let Some(ident) = &field.ident { ident.to_string() } else { f_idx.to_string() };

            for attr in &field.attrs {
                if attr.path().is_ident("rd_ui") {
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("label") {
                            let value = meta.value()?;
                            let lit: syn::LitStr = value.parse()?;
                            f_label = lit.value();
                        }
                        Ok(())
                    });
                }
            }

            let field_base_ident = if let Some(ident) = &field.ident {
                ident.clone()
            } else {
                format_ident!("f{}", f_idx)
            };

            let form_field_ident = format_ident!("v{}_{}", v_idx, field_base_ident);

            original_field_binds.push(f_bind);
            field_tys.push(ty);
            field_labels.push(f_label);
            form_field_idents.push(form_field_ident);
        }

        let is_named = matches!(variant.fields, Fields::Named(_));
        let pat = if is_named {
            quote! { { #( #original_field_binds ),* } }
        } else {
            quote! { ( #( #original_field_binds ),* ) }
        };

        let ignore_pat = if is_named {
            quote! { { .. } }
        } else {
            quote! { ( .. ) }
        };

        from_arms.push(quote! { #name::#v_ident #ignore_pat => #kind_name::#v_ident });

        for (form_field_ident, ty) in form_field_idents.iter().zip(field_tys.iter()) {
            form_fields.push(quote! {
                #form_field_ident: ::rd_ui::gpui::Entity<::rd_ui::InputState<<#ty as ::rd_ui::AutoInput>::Delegate>>
            });
        }

        fields_match_arms.push(quote! {
            #kind_name::#v_ident => {
                #(
                    fields.push(::rd_ui::FormField::new(
                        #field_labels,
                        ::rd_ui::Input::new(self.#form_field_idents.clone()),
                        cx
                    ));
                )*
            }
        });

        extract_match_arms.push(quote! {
            #kind_name::#v_ident => {
                #(
                    let #original_field_binds = self.#form_field_idents.read(cx).delegate().value_or_default(cx).clone();
                )*
                Some(#name::#v_ident #pat)
            }
        });

        let init_tuple_names: Vec<_> =
            form_field_idents.iter().map(|ident| format_ident!("init_{}", ident)).collect();

        init_extractions.push(quote! {
            let ( #( #init_tuple_names ),* ) = match &initial_value {
                #name::#v_ident #pat => {
                    ( #( Some(#original_field_binds.clone()) ),* )
                }
                _ => ( #( None::<#field_tys> ),* )
            };
        });

        for (form_field_ident, (init_name, ty)) in
            form_field_idents.iter().zip(init_tuple_names.iter().zip(field_tys.iter()))
        {
            field_builds.push(quote! {
                let #form_field_ident = {
                    let val = #init_name.unwrap_or_default();
                    <#ty as ::rd_ui::AutoInput>::build_input(val, window, cx)
                };
            });
            form_struct_inits.push(quote! { #form_field_ident });
        }
    }

    quote! {
        #[derive(Clone, Copy, PartialEq, ::rd_ui::Input)]
        #vis enum #kind_name {
            #( #kind_variants ),*
        }

        impl From<#name> for #kind_name {
            fn from(value: #name) -> Self {
                match value {
                    #( #from_arms ),*
                }
            }
        }

        #vis struct #form_name {
            kind: ::rd_ui::gpui::Entity<::rd_ui::InputState<::rd_ui::Picker<#kind_name>>>,
            #( #form_fields, )*
        }

        impl ::rd_ui::FormDelegate for #form_name {
            type Data = #name;

            fn fields(&self, cx: &mut ::rd_ui::gpui::App) -> Vec<::rd_ui::FormField> {
                let mut fields = vec![
                    ::rd_ui::FormField::new(
                        "Kind",
                        ::rd_ui::Input::new(self.kind.clone()),
                        cx
                    ).with_label_hidden(true)
                ];

                match self.kind.read(cx).value(cx) {
                    #( #fields_match_arms, )*
                }

                fields
            }

            fn extract_data(&self, cx: &::rd_ui::gpui::App) -> Option<Self::Data> {
                use ::rd_ui::InputDelegate as _;

                let kind = self.kind.read(cx).value(cx).clone();
                match kind {
                    #( #extract_match_arms, )*
                }
            }
        }

        impl ::rd_ui::AutoInput for #name {
            type Delegate = ::rd_ui::Form<#form_name>;

            fn build_input(
                initial_value: Self,
                window: &mut ::rd_ui::gpui::Window,
                cx: &mut ::rd_ui::gpui::App,
            ) -> ::rd_ui::gpui::Entity<::rd_ui::InputState<Self::Delegate>> {
                use ::rd_ui::gpui::AppContext as _;

                let kind_entity = {
                    let init_kind = #kind_name::from(initial_value.clone());
                    <#kind_name as ::rd_ui::AutoInput>::build_input(init_kind, window, cx)
                };

                #( #init_extractions )*

                #( #field_builds )*

                cx.new(move |cx| {
                    let delegate = #form_name {
                        kind: kind_entity,
                        #( #form_struct_inits, )*
                    };

                    let form = ::rd_ui::Form::new(delegate, cx.focus_handle(), window, cx);
                    ::rd_ui::InputState::new(form, window, cx)
                })
            }
        }
    }
}
