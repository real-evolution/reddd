use darling::{FromDeriveInput, FromField, ToTokens};
use quote::quote;

#[derive(Clone, Debug, FromField)]
#[darling(forward_attrs(main_field))]
struct ValueTypeField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(forward_attrs(allow, doc, cfg))]
#[darling(attributes(main_field), supports(struct_any))]
pub(super) struct ValueType {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), ValueTypeField>,
}

impl ValueTypeField {
    fn is_main_field(&self) -> bool {
        self.attrs.iter().any(|a| {
            a.path
                .get_ident()
                .map(|id| id == "main_field")
                .unwrap_or(false)
        })
    }
}

impl ToTokens for ValueType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ValueType {
            ref ident,
            ref generics,
            ref data,
        } = *self;

        let (imp, ty, wher) = generics.split_for_impl();

        let fields: Vec<_> = data
            .as_ref()
            .take_struct()
            .expect("only structs are supported")
            .fields
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                (
                    f.is_main_field(),
                    f.ident.as_ref().map(|id| quote!(#id)).unwrap_or_else(
                        || {
                            let i = syn::Index::from(i);

                            quote!(#i)
                        },
                    ),
                    &f.ty,
                )
            })
            .collect();

        let (field_id, field_ty) = if fields.is_empty() {
            panic!("field-less structs are not supported");
        } else if fields.iter().filter(|(m, _, _)| *m).count() > 1 {
            panic!("only one field can be set as the main field");
        } else {
            let (_, field_id, field_ty) = fields
                .iter()
                .find(|(m, _, _)| *m)
                .unwrap_or(fields.first().unwrap());

            (field_id, field_ty)
        };

        tokens.extend(quote::quote! {
            impl #imp ValueType for #ident #ty #wher {
                type Value = #field_ty;

                fn value(self) -> Self::Value {
                    self.#field_id
                }

                fn value_ref(&self) -> &Self::Value {
                    &self.#field_id
                }
            }

            impl #imp PartialEq for #ident #ty #wher {
                fn eq(&self, other: &Self) -> bool {
                    matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Equal))
                }
            }

            impl #imp PartialOrd for #ident #ty #wher {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.#field_id.partial_cmp(&other.#field_id)
                }
            }
        });
    }
}
