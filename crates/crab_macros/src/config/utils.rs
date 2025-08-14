use syn::{Attribute, Expr, ExprLit, Field, Lit, Meta, Variant};

pub trait SynAttr {
    fn attrs(&self) -> &[Attribute];
}

impl SynAttr for Variant {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl SynAttr for Field {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

pub fn collect_docs<T: SynAttr>(variant: &T) -> String {
    variant
        .attrs()
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc")
                && let Meta::NameValue(meta) = &attr.meta
                && let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str), ..
                }) = &meta.value
            {
                return Some(lit_str.value());
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n")
}
