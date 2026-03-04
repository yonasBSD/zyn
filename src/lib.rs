pub use zyn_core::*;

#[cfg(feature = "derive")]
pub use zyn_derive::*;

#[cfg(feature = "ext")]
pub mod ext {
    pub use zyn_ext::*;
}

pub use proc_macro2;
pub use quote;

#[cfg(feature = "syn")]
pub use syn;

/// Common imports for working with zyn templates.
///
/// ```ignore
/// use zyn::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        Camel, Fmt, Ident, Kebab, Lower, Pascal, Pipe, Render, Screaming, Snake, Upper,
    };

    #[cfg(feature = "derive")]
    pub use zyn_derive::*;

    #[cfg(feature = "ext")]
    pub use zyn_ext::*;
}
