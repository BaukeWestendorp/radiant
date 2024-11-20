use darling::{FromDeriveInput, FromMeta, FromVariant};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Ident, Path, Type, Variant};

#[derive(FromDeriveInput)]
#[darling(supports(enum_any), attributes(node_kind))]
struct Attrs {
    graph_definition: Type,
    processing_context: Type,
}

#[derive(FromVariant)]
#[darling(attributes(node))]
struct VariantMeta {
    name: String,
    category: Path,
    #[darling(default)]
    processor: Option<Path>,
}

#[derive(FromMeta)]
struct Input {
    label: String,
    data_type: Path,
    control: Path,
}

#[derive(FromMeta)]
struct ComputedOutput {
    label: String,
    data_type: Path,
}

#[derive(FromMeta)]
struct ConstantOutput {
    label: String,
    data_type: Path,
    control: Path,
}

struct NodeKindVariant {
    variant: Variant,
    meta: VariantMeta,
    inputs: Vec<Input>,
    computed_outputs: Vec<ComputedOutput>,
    constant_outputs: Vec<ConstantOutput>,
}

impl NodeKindVariant {
    pub fn output_labels(&self) -> Vec<String> {
        let mut labels = vec![];
        for output in &self.computed_outputs {
            labels.push(output.label.clone());
        }
        for output in &self.constant_outputs {
            labels.push(output.label.clone());
        }
        labels
    }
}

impl TryFrom<Variant> for NodeKindVariant {
    type Error = darling::Error;

    fn try_from(variant: Variant) -> Result<Self, Self::Error> {
        let meta = VariantMeta::from_variant(&variant)?;
        let inputs = parse_input_attrs(&variant)?;
        let computed_outputs = parse_computed_output_attrs(&variant)?;
        let constant_outputs = parse_constant_output_attrs(&variant)?;

        Ok(NodeKindVariant {
            variant,
            meta,
            inputs,
            computed_outputs,
            constant_outputs,
        })
    }
}

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let attrs: Attrs = match Attrs::from_derive_input(&input) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let graph_def = &attrs.graph_definition;
    let name = &input.ident;
    let proc_ctx = attrs.processing_context;

    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            match variants
                .into_iter()
                .map(NodeKindVariant::try_from)
                .collect::<darling::Result<Vec<_>>>()
            {
                Ok(variants) => variants,
                Err(err) => return err.write_errors().into(),
            }
        }
        _ => {
            return syn::Error::new(input.span(), "A NodeKind can only be an enum")
                .to_compile_error()
                .into()
        }
    };

    let builders = gen_builders(&variants);
    let processing_output_types = gen_processing_output_types(&variants, &graph_def);
    let processors = gen_processors(&variants, graph_def.clone());

    let extracted = quote! {
        impl flow::NodeKind<#graph_def> for #name {
            type ProcessingContext = #proc_ctx;

            fn build(&self, graph: &mut flow::Graph<#graph_def>, node_id: flow::NodeId) {
                match self {
                    #(#builders)*
                }
            }

            fn process(
                &self,
                node_id: flow::NodeId,
                context: &mut Self::ProcessingContext,
                graph: &flow::Graph<#graph_def>,
            ) -> flow::Result<flow::ProcessingResult<#graph_def>> {
                use flow::Value as _;

                let node = graph.node(node_id);
                let mut processing_result = flow::ProcessingResult::new();

                let mut value_for_input =
                    |node: &flow::Node<#graph_def>, input_name: &str| -> flow::Result<Value> {
                        let input = graph.input(node.input(input_name).id);
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
                    };

                match self {
                    #(#processors)*
                }

                Ok(processing_result)
            }
        }

        #(#processing_output_types)*
    };

    extracted.into()
}

fn gen_builders(variants: &[NodeKindVariant]) -> Vec<TokenStream> {
    variants
        .iter()
        .map(move |variant| {
            let variant_ident = &variant.variant.ident;

            let input_builders = variant.inputs.iter().map(|input| {
                let label = &input.label;
                let data_type = &input.data_type;
                let control = &input.control;

                quote! {
                    graph.add_input(
                        node_id,
                        #label.to_string(),
                        #data_type,
                        flow::InputParameterKind::EdgeOrConstant {
                            value: #data_type.default_value(),
                            control: #control,
                        }
                    );
                }
            });

            let computed_output_builders = variant.computed_outputs.iter().map(|output| {
                let label = &output.label;
                let data_type = &output.data_type;

                quote! {
                    graph.add_output(
                        node_id,
                        #label.to_string(),
                        #data_type,
                        flow::OutputParameterKind::Computed,
                    );
                }
            });

            let constant_output_builders = variant.constant_outputs.iter().map(|output| {
                let label = &output.label;
                let data_type = &output.data_type;
                let control = &output.control;

                quote! {{
                    use flow::DataType as _;
                    graph.add_output(
                        node_id,
                        #label.to_string(),
                        #data_type,
                        flow::OutputParameterKind::Constant {
                            value: #data_type.default_value(),
                            control: #control,
                        }
                    );
                }}
            });

            quote! {
               Self::#variant_ident => {
                    use flow::DataType as _;
                    #(#input_builders)*
                    #(#computed_output_builders)*
                    #(#constant_output_builders)*
                }
            }
        })
        .collect()
}

fn gen_processors(variants: &[NodeKindVariant], graph_def: Type) -> Vec<TokenStream> {
    variants
        .into_iter()
        .map(move |variant| {
            let variant_ident = &variant.variant.ident;

            let Some(processor_path) = &variant.meta.processor else {
                return quote! { Self::#variant_ident => {} };
            };

            let parameters = variant.inputs.iter().map(|input| {
                let label = &input.label;
                let ident = format_ident!("{}", &label);
                let data_type = &input
                    .data_type
                    .segments
                    .last()
                    .expect("DataType and Value should have matching last path segments")
                    .ident;

                quote! {
                    {
                        type __Value = <#graph_def as flow::GraphDefinition>::Value;
                        let __Value::#data_type(#ident) = value_for_input(node, #label)? else {
                            return Err(flow::FlowError::CastFailed);
                        };
                        #ident
                    }
                }
            });

            let output_type_name = processing_output_type_name(&variant.variant);
            let output_labels = variant.output_labels();
            let output_label_idents = output_labels.iter().map(|label| format_ident!("{}", label));
            let value_setters = output_labels.iter().map(|label| {
                let label_ident = format_ident!("{}", label);
                quote! {
                    processing_result.set_value(
                        node.output(#label).id,
                        <#graph_def as flow::GraphDefinition>::Value::from(#label_ident)
                    );
                }
            });

            quote! {
                Self::#variant_ident => {
                    let #output_type_name {
                        #(#output_label_idents),*
                    } = #processor_path(
                        #(#parameters,)*
                        context
                    );

                    #(#value_setters)*
                }
            }
        })
        .collect()
}

fn gen_processing_output_types(variants: &[NodeKindVariant], graph_def: &Type) -> Vec<TokenStream> {
    variants
        .iter()
        .map(|variant| {
            let name = processing_output_type_name(&variant.variant);

            let fields = variant.output_labels().into_iter().map(|label| {
                let field_name = format_ident!("{}", label);
                quote! {
                    pub #field_name: <#graph_def as flow::GraphDefinition>::Value,
                }
            });

            quote! {
                struct #name {
                    #(#fields)*
                }
            }
        })
        .collect()
}

fn parse_input_attrs(variant: &Variant) -> darling::Result<Vec<Input>> {
    get_attrs_with_ident(variant, "input")
        .into_iter()
        .map(|a| Input::from_meta(&a.meta))
        .collect()
}

fn parse_computed_output_attrs(variant: &Variant) -> darling::Result<Vec<ComputedOutput>> {
    get_attrs_with_ident(&variant, "computed_output")
        .map(|a| ComputedOutput::from_meta(&a.meta))
        .collect()
}

fn parse_constant_output_attrs(variant: &Variant) -> darling::Result<Vec<ConstantOutput>> {
    get_attrs_with_ident(&variant, "constant_output")
        .map(|a| ConstantOutput::from_meta(&a.meta))
        .collect()
}

fn get_attrs_with_ident<'a>(
    variant: &'a Variant,
    ident: &'a str,
) -> impl Iterator<Item = &'a Attribute> {
    variant.attrs.iter().filter(|a| a.path().is_ident(ident))
}

fn processing_output_type_name(variant: &Variant) -> Ident {
    format_ident!("{}ProcessingOutput", variant.ident)
}
