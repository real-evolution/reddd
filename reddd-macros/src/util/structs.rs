use super::FieldExt;

pub(crate) trait StructExt<F> {
    fn get_fields(&self) -> Vec<&F>;
    fn get_fields_by_attr(&self, attr: &str) -> Vec<&F>;
    fn get_field_by_id(&self, id: &str) -> Option<&F>;
    fn get_field_by_attr(&self, attr: &str) -> Option<&F>;
    fn get_field_by_attr_or_id(&self, attr: &str, id: &str) -> Option<&F>;
}

impl<F> StructExt<F> for darling::ast::Data<(), F>
where
    F: FieldExt,
{
    fn get_fields(&self) -> Vec<&F> {
        self.as_ref()
            .take_struct()
            .expect("only structs are supported")
            .fields
    }

    fn get_fields_by_attr(&self, attr: &str) -> Vec<&F> {
        self.get_fields()
            .into_iter()
            .filter(|f| f.has_attribute(attr))
            .collect()
    }

    fn get_field_by_id(&self, id: &str) -> Option<&F> {
        self.get_fields().into_iter().find(|f| match f.id() {
            | Some(fid) => fid == id,
            | None => false,
        })
    }

    fn get_field_by_attr(&self, attr: &str) -> Option<&F> {
        let fields = self.get_fields_by_attr(attr);

        if fields.len() > 1 {
            panic!("only one field can be annotated with #[{attr}]");
        }

        fields.first().copied()
    }

    fn get_field_by_attr_or_id(&self, attr: &str, id: &str) -> Option<&F> {
        self.get_field_by_attr(attr).or(self.get_field_by_id(id))
    }
}
