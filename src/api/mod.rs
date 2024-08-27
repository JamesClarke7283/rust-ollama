#[cfg(feature = "async")]
pub mod list {
    pub use super::r#async::list::list_models;
}

#[cfg(not(feature = "async"))]
pub mod list {
    pub use super::sync::list::list_models;
}

#[cfg(feature = "async")]
pub mod r#async;

#[cfg(not(feature = "async"))]
pub mod sync;
