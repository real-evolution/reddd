use darling::{FromDeriveInput, FromField, ToTokens};

#[derive(Clone, Debug, FromField)]
#[darling(attributes(main_field))]
pub(super) struct MainField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

#[derive(Debug, FromDeriveInput)]
#[darling(forward_attrs(allow, doc, cfg))]
#[darling(supports(struct_any))]
pub(super) struct ValueType {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), MainField>,
}

impl ToTokens for ValueType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ValueType {
            ref ident,
            ref generics,
            ref data,
        } = *self;

        let (imp, ty, wher) = generics.split_for_impl();

        let fields = data
            .as_ref()
            .take_struct()
            .expect("only structs are supported")
            .fields;

        if fields.len() != 1 {
            panic!("only one field should be annotated as the main field");
        }

        let MainField {
            ident: field_ident,
            ty: field_ty,
        } = fields.first().expect("at least one field is required");

        tokens.extend(quote::quote! {
            impl #imp ValueType for #ident #ty #wher {
                type Value = #field_ty;

                fn value(self) -> Self::Value {
                    self.#field_ident
                }

                fn value_ref(&self) -> &Self::Value {
                    &self.#field_ident
                }
            }
        });
    }
}
