use crate::util::StructExt;
use darling::{FromDeriveInput, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(forward_attrs(allow, doc, cfg))]
#[darling(attributes(id_field, created_at_field), supports(struct_named))]
pub(super) struct Entity {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), syn::Field>,
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Entity {
            ref ident,
            ref generics,
            ref data,
        } = *self;

        let (imp, ty, wher) = generics.split_for_impl();

        let id_field = data
            .get_field_by_attr_or_id("id_field", "id")
            .expect("`id` field is required");

        let created_at_field = data
            .get_field_by_attr_or_id("created_at_field", "created_at")
            .expect("`created_at` field is required");

        let (id_ident, id_ty) = (id_field.ident.clone().unwrap(), &id_field.ty);
        let created_at_ident = &created_at_field.ident.clone().unwrap();

        tokens.extend(quote::quote! {
            impl #imp Entity for #ident #ty #wher {
                type Key = #id_ty;

                fn id(&self) -> &Self::Key {
                    &self.#id_ident
                }

                fn created_at(&self) -> &DateTime<Utc> {
                    &self.#created_at_ident
                }
            }
        });
    }
}
