pub use zyn_core::*;

#[cfg(feature = "derive")]
pub use zyn_derive::*;

/// Common imports for working with zyn templates.
///
/// ```ignore
/// use zyn::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{Camel, Kebab, Lower, Pascal, Pipe, Render, Screaming, Snake, Upper};

    #[cfg(feature = "derive")]
    pub use zyn_derive::{element, pipe, zyn};
}
