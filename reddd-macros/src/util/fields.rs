pub(crate) trait FieldExt {
    fn id(&self) -> Option<syn::Ident>;

    fn has_attribute(&self, attr: &str) -> bool;
}

macro_rules! impl_field_ext {
    ($type:ty) => {
        impl crate::util::FieldExt for $type {
            fn id(&self) -> Option<syn::Ident> {
                self.ident.clone()
            }

            fn has_attribute(&self, attr: &str) -> bool {
                self.attrs.iter().any(|a| {
                    a.path().get_ident().map(|id| id == attr).unwrap_or(false)
                })
            }
        }
    };
}

impl_field_ext!(syn::Field);
