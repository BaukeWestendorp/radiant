use darling::{FromDeriveInput, FromMeta, FromVariant};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Expr, Ident, Type, Variant};

#[derive(FromDeriveInput)]
#[darling(attributes(node_kind))]
struct Attrs {
    graph_definition: Type,
    processing_context: Type,
}

#[derive(Debug, FromMeta)]
struct VariantInput {
    label: String,
    data_type: Ident,

    #[darling(default)]
    default_value: Option<Expr>,

    control: Ident,
}

#[derive(Debug, FromMeta)]
struct VariantComputedOutput {
    label: String,
    data_type: Ident,
}

#[derive(Debug, FromMeta)]
struct VariantConstantOutput {
    label: String,
    data_type: Ident,

    #[darling(default)]
    default_value: Option<Expr>,

    control: Ident,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(meta))]
struct VariantMeta {
    name: String,
    category: Ident,
    processor: Ident,
}

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let attrs = Attrs::from_derive_input(&input)?;

    let variants = match &input.data {
        Data::Enum(data) => data.variants.clone().into_iter().collect::<Vec<_>>(),
        _ => panic!("Only enums are supported"),
    };

    let type_helpers = gen_type_helpers(&attrs);
    let impl_node_kind = gen_impl_node_kind(&input, &variants, &attrs);
    let processor_output_types = gen_processor_types(&variants);
    let impl_visual_node_kind = gen_impl_visual_node_kind(&variants);

    let expansion = quote! {
        #type_helpers
        #impl_node_kind
        #processor_output_types
        #impl_visual_node_kind
    };

    Ok(expansion)
}

fn gen_type_helpers(attrs: &Attrs) -> TokenStream {
    let graph_definition = &attrs.graph_definition;

    quote! {
        type __GraphNodeKind = <#graph_definition as flow::GraphDefinition>::NodeKind;
        type __GraphDataType = <#graph_definition as flow::GraphDefinition>::DataType;
        type __GraphValue = <#graph_definition as flow::GraphDefinition>::Value;
        type __GraphControl = <#graph_definition as flow::GraphDefinition>::Control;
    }
}

fn gen_impl_node_kind(input: &DeriveInput, variants: &[Variant], attrs: &Attrs) -> TokenStream {
    fn gen_builder(variant: &Variant) -> TokenStream {
        fn gen_input(input_attr: &VariantInput) -> TokenStream {
            let VariantInput {
                label,
                data_type,
                default_value,
                control,
            } = input_attr;

            quote! {
                graph.add_input(
                    node_id,
                    #label.to_string(),
                    __GraphDataType::#data_type,
                    flow::InputParameterKind::EdgeOrConstant {
                        value: __GraphValue::#data_type(#default_value),
                        control: __GraphControl::#control,
                    },
                );
            }
        }

        fn gen_computed_output_for_builder(output_attr: &VariantComputedOutput) -> TokenStream {
            let VariantComputedOutput { label, data_type } = output_attr;

            quote! {
                graph.add_output(
                     node_id,
                     #label.to_string(),
                    __GraphDataType::#data_type,
                    flow::OutputParameterKind::Computed,
                );
            }
        }

        fn gen_constant_output_for_builder(output_attr: &VariantConstantOutput) -> TokenStream {
            let VariantConstantOutput {
                label,
                data_type,
                default_value,
                control,
            } = output_attr;

            quote! {
                graph.add_output(
                    node_id,
                    #label.to_string(),
                    __GraphDataType::#data_type,
                    flow::OutputParameterKind::Constant {
                        value: __GraphValue::#data_type(#default_value),
                        control: __GraphControl::#control,
                    },
                );
            }
        }

        let name = &variant.ident;

        let input_attrs = parse_input_attrs(&variant);
        let inputs = input_attrs.iter().map(gen_input);

        let computed_output_attrs = parse_computed_output_attrs(&variant);
        let computed_outputs = computed_output_attrs
            .iter()
            .map(gen_computed_output_for_builder);

        let constant_output_attrs = parse_constant_output_attrs(&variant);
        let constant_outputs = constant_output_attrs
            .iter()
            .map(gen_constant_output_for_builder);

        quote! {
            Self::#name => {
                #(#inputs)*
                #(#computed_outputs)*
                #(#constant_outputs)*
            }
        }
    }

    fn gen_processor(variant: &Variant) -> TokenStream {
        let name = &variant.ident;
        let meta = parse_variant_meta(variant);
        let processor = &meta.processor;

        let input_declarations = parse_input_attrs(variant).into_iter().map(|input| {
            let label = format_ident!("{}", input.label);

            quote! {
                #label: {
                    let input = graph.input(node.input(stringify!(#label)).id);
                    let connection_id = graph.edge_source(input.id());
                    let value = match connection_id {
                        None => {
                            let flow::InputParameterKind::EdgeOrConstant { value, .. } =
                                graph.input(input.id()).kind.clone();
                            value
                        }
                        Some(id) => graph.get_output_value(&id, context)?.clone(),
                    };

                    let value = value.try_cast_to(&input.data_type())?;
                    Ok(value)
                }?
            }
        });

        let processor_input_ident = processor_input_ident(variant);

        let processing_result_modifications =
            parse_variant_output_labels(variant)
                .into_iter()
                .map(|label| {
                    let label_ident = format_ident!("{}", label);
                    quote! {
                        processing_result.set_output_value(
                            node.output(#label).id,
                            output.#label_ident,
                        );
                    }
                });

        quote! {
            Self::#name => {
                let output = #processor(#processor_input_ident {
                   #(#input_declarations),*
                }, context)?;



                let mut processing_result = flow::ProcessingResult::new();
                #(#processing_result_modifications)*
                Ok(processing_result)
            }
        }
    }

    let Attrs {
        graph_definition,
        processing_context,
        ..
    } = &attrs;

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;

    let builders = variants.iter().map(gen_builder);
    let processors = variants.iter().map(gen_processor);

    quote! {
        impl #impl_generics flow::NodeKind<#graph_definition> for #name #type_generics #where_clause {
            type ProcessingContext = #processing_context;

            fn build(&self, graph: &mut flow::Graph<#graph_definition>, node_id: flow::NodeId) {
                match self {
                    #(#builders)*
                }
            }

            fn process(
                &self,
                node_id: flow::NodeId,
                context: &mut Self::ProcessingContext,
                graph: &flow::Graph<#graph_definition>,
            ) -> Result<flow::ProcessingResult<#graph_definition>, flow::GraphError> {
                let node = graph.node(node_id);
                match self {
                    #(#processors)*
                }
            }
        }
    }
}

fn gen_processor_types(variants: &[Variant]) -> TokenStream {
    fn gen_processor_input_type(variant: &Variant) -> TokenStream {
        let name = processor_input_ident(variant);

        let input_labels = parse_variant_input_labels(variant);

        let mut fields = vec![];
        for label in input_labels {
            let ident = format_ident!("{}", label);

            fields.push(quote! {
                pub #ident: __GraphValue,
            })
        }

        quote! {
            #[derive(Debug)]
            struct #name {
                #(#fields)*
            }
        }
    }

    fn gen_processor_output_type(variant: &Variant) -> TokenStream {
        let name = processor_output_ident(variant);

        let output_labels = parse_variant_output_labels(variant);

        let mut fields = vec![];
        for label in output_labels {
            let ident = format_ident!("{}", label);

            fields.push(quote! {
                pub #ident: __GraphValue,
            })
        }

        quote! {
            #[derive(Debug)]
            struct #name {
                #(#fields)*
            }
        }
    }

    let input_types = variants.iter().map(gen_processor_input_type);
    let output_types = variants.iter().map(gen_processor_output_type);

    quote! {
        #(#input_types)*
        #(#output_types)*
    }
}

fn gen_impl_visual_node_kind(variants: &[Variant]) -> TokenStream {
    let mut names = vec![];
    let mut categories = vec![];
    let mut all = vec![];

    for variant in variants {
        let meta = parse_variant_meta(variant);

        let var = &variant.ident;
        let name = &meta.name;
        let cat = &meta.category;

        names.push(quote! {
            Self::#var => #name,
        });

        categories.push(quote! {
            Self::#var => __GraphNodeCategory::#cat,
        });

        all.push(quote! {
            Self::#var
        })
    }

    quote! {
        impl flow_gpui::VisualNodeKind for __GraphNodeKind {
            type Category = __GraphNodeCategory;

            fn name(&self) -> &str {
                match self {
                    #(#names)*
                }
            }

            fn category(&self) -> Self::Category {
                match self {
                    #(#categories)*
                }
            }

            fn all() -> impl Iterator<Item = Self> {
                vec![#(#all),*].into_iter()
            }
        }
    }
}

fn parse_variant_meta(variant: &Variant) -> VariantMeta {
    VariantMeta::from_variant(variant)
        .expect("NodeKind variant should have a #[meta(...)] attribute")
}

fn parse_input_attrs(variant: &Variant) -> Vec<VariantInput> {
    variant
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("input"))
        .map(|attr| VariantInput::from_meta(&attr.meta).unwrap())
        .collect::<Vec<_>>()
}

fn parse_computed_output_attrs(variant: &Variant) -> Vec<VariantComputedOutput> {
    variant
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("computed_output"))
        .map(|attr| VariantComputedOutput::from_meta(&attr.meta).unwrap())
        .collect::<Vec<_>>()
}

fn parse_constant_output_attrs(variant: &Variant) -> Vec<VariantConstantOutput> {
    variant
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("constant_output"))
        .map(|attr| VariantConstantOutput::from_meta(&attr.meta).unwrap())
        .collect::<Vec<_>>()
}

fn parse_variant_input_labels(variant: &Variant) -> impl Iterator<Item = String> {
    parse_input_attrs(variant)
        .into_iter()
        .map(|input| input.label)
}

fn parse_variant_output_labels(variant: &Variant) -> impl Iterator<Item = String> {
    let constant_output_labels = parse_constant_output_attrs(variant)
        .into_iter()
        .map(|output| output.label);

    parse_computed_output_attrs(variant)
        .into_iter()
        .map(|output| output.label)
        .chain(constant_output_labels)
}

fn processor_input_ident(variant: &Variant) -> Ident {
    format_ident!("{}ProcessorInput", variant.ident)
}

fn processor_output_ident(variant: &Variant) -> Ident {
    format_ident!("{}ProcessorOutput", variant.ident)
}
