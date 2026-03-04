use syn::Attribute;

use crate::meta::Args;

pub trait AttrExt {
    fn is(&self, name: &str) -> bool;
    fn args(&self) -> syn::Result<Args>;
}

impl AttrExt for Attribute {
    fn is(&self, name: &str) -> bool {
        self.path().is_ident(name)
    }

    fn args(&self) -> syn::Result<Args> {
        self.parse_args::<Args>()
    }
}

pub trait AttrsExt {
    fn find_attr(&self, name: &str) -> Option<&Attribute>;
    fn find_args(&self, name: &str) -> syn::Result<Option<Args>>;
    fn has_attr(&self, name: &str) -> bool;
    fn merge_args(&self, name: &str) -> syn::Result<Args>;
}

impl AttrsExt for [Attribute] {
    fn find_attr(&self, name: &str) -> Option<&Attribute> {
        self.iter().find(|a| a.is(name))
    }

    fn find_args(&self, name: &str) -> syn::Result<Option<Args>> {
        match self.find_attr(name) {
            Some(attr) => Ok(Some(attr.args()?)),
            None => Ok(None),
        }
    }

    fn has_attr(&self, name: &str) -> bool {
        self.iter().any(|a| a.is(name))
    }

    fn merge_args(&self, name: &str) -> syn::Result<Args> {
        let mut result = Args::new();

        for attr in self.iter().filter(|a| a.is(name)) {
            let args = attr.args()?;
            result.extend(args);
        }

        Ok(result)
    }
}
