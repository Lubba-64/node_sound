#[macro_use]
pub mod macros {
    macro_rules! crate_version {
        () => {
            env!("CARGO_PKG_VERSION")
        };
    }

    pub(crate) use crate_version;
}
