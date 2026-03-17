use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{
    ExprClosure,
    GenericArgument,
    Ident,
    Path,
    PathArguments,
    Result,
    Token,
    Type,
    TypePath,
    parse::{Parse, ParseStream},
    parse_macro_input,
    parse_quote,
    spanned::Spanned
};

// use crate::utils::{Antcore, string_to_pascal_case};

#[derive(Debug)]
struct ExportConversionInput {
    implementations: Vec<ExportConversionImplementation>,
    r#type:          Type
}

#[derive(Debug)]
struct ExportConversionImplementation {
    closure:     ExprClosure,
    name:        Ident,
    return_type: Type,
    r#trait:     Type
}

impl Parse for ExportConversionInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let r#type = input.parse::<Type>()?;
        input.parse::<Token![,]>()?;

        let mut implementations = Vec::new();

        {
            let r#trait = input.parse::<Type>()?;

            if r#trait != parse_quote! { Into<IntermediaryExport> } {
                return Err(input.error(
                    "first implementation must be Into<IntermediaryExport>"
                ));
            }

            let closure = input.parse::<ExprClosure>()?;

            if closure.inputs.first().is_none() {
                return Err(input.error("closure must have at least one input"));
            }

            let (into_span, type_span) = match &r#trait {
                Type::Path(TypePath {
                    path: Path { segments, .. },
                    ..
                }) if let Some(segment) = segments.first() => (
                    segment.ident.span(),
                    extract_angle_bracketed_type(&r#trait)
                        .expect("known valid")
                        .span()
                ),
                _ => {
                    unreachable!();
                }
            };

            let return_type = Ident::new("IntermediaryExport", type_span);

            let into_intermediary = Ident::new("IntoIntermediary", into_span);

            implementations.push(ExportConversionImplementation {
                closure,
                name: Ident::new("into_intermediary", Span::call_site()),
                return_type: parse_quote! { #return_type },
                r#trait: parse_quote! { crate::conversion::#into_intermediary }
            });
        }

        {
            let r#trait = input.parse::<Type>()?;

            if r#trait != parse_quote! { From<IntermediaryExport> } {
                return Err(input.error(
                    "second implementation must be From<IntermediaryExport>"
                ));
            }

            let closure = input.parse::<ExprClosure>()?;

            if closure.inputs.first().is_none() {
                return Err(input.error("closure must have at least one input"));
            }

            let (logged_from_span, inner_type_span) = match &r#trait {
                Type::Path(TypePath {
                    path: Path { segments, .. },
                    ..
                }) if let Some(segment) = segments.first() => (
                    segment.ident.span(),
                    extract_angle_bracketed_type(&r#trait)
                        .expect("known valid")
                        .span()
                ),
                _ => {
                    unreachable!();
                }
            };

            let inner_type = Ident::new("IntermediaryExport", inner_type_span);
            let logged_from = Ident::new("LoggedFrom", logged_from_span);

            implementations.push(ExportConversionImplementation {
                closure,
                name: Ident::new("logged_from", Span::call_site()),
                return_type: parse_quote! { Self },
                r#trait: parse_quote! {
                    crate::conversion::#logged_from<#inner_type>
                }
            });
        }

        // ? other implementations are optional
        while !input.is_empty() {
            let r#trait = input.parse::<Type>()?;

            // ? must be provided as From<T>
            // ? rewrite as LoggedFrom<T>

            // ? the things we do for span preservation

            let Some(inner_type) = extract_angle_bracketed_type(&r#trait)
            else {
                return Err(input.error(
                    "additional implementations must be provided as From<T>"
                ));
            };

            let logged_from = Ident::new("LoggedFrom", match &r#trait {
                Type::Path(TypePath {
                    path: Path { segments, .. },
                    ..
                }) if let Some(segment) = segments.first() &&
                    segment.ident == "From" =>
                {
                    segment.ident.span()
                }
                _ => {
                    return Err(input.error(
                        "additional implementations must be provided as From<T>"
                    ));
                }
            });

            let closure = input.parse::<ExprClosure>()?;

            if closure.inputs.first().is_none() {
                return Err(input.error("closures must have at least one input"));
            }

            implementations.push(ExportConversionImplementation {
                closure,
                name: Ident::new("logged_from", Span::call_site()),
                return_type: parse_quote! { Self },
                r#trait: parse_quote! {
                    crate::conversion::#logged_from<#inner_type>
                }
            });

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            implementations,
            r#type
        })
    }
}

impl ToTokens for ExportConversionInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            implementations,
            r#type
        } = self;

        let underscore = parse_quote! { _ };

        let implementations = implementations
            .iter()
            .map(
                |ExportConversionImplementation {
                     closure,
                     name,
                     return_type,
                     r#trait
                 }| {
                    // ? why did i choose to do it this way
                    // ? i just liked the closure syntax
                    // ? aaaaaaaaaagony
                    let input_1 = closure
                        .inputs
                        .first()
                        .expect("closure should have at least one input");

                    let input_2 = closure.inputs.get(1).unwrap_or(&underscore);

                    let input_1_type = extract_angle_bracketed_type(r#trait)
                        .map_or_else(|| quote! {}, |type_| quote! { :#type_ });

                    let body = &*closure.body;

                    quote! {
                        impl #r#trait for #r#type {
                            fn #name(
                                #input_1 #input_1_type,
                                #input_2: &mut Vec<String>
                            ) -> #return_type #body
                        }
                    }
                }
            )
            .collect::<Vec<_>>();


        tokens.extend(quote! {
           impl crate::conversion::LoggedFrom<Self> for #r#type {
               fn logged_from(value: Self, _: &mut Vec<String>) -> Self {
                   value
               }
           }

           #(#implementations)*
        });
    }
}

pub fn export_conversion(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as ExportConversionInput);

    quote! { #input }.into()
}


fn extract_angle_bracketed_type(type_: &Type) -> Option<&Type> {
    let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = type_
    else {
        return None;
    };

    let segment = segments.last()?;

    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };

    let Some(GenericArgument::Type(inner_type)) = arguments.args.first() else {
        return None;
    };

    Some(inner_type)
}
