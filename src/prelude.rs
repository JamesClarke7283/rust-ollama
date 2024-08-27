#[cfg(feature = "async")]
pub use crate::api::client::Client;

#[cfg(feature = "async")]
pub use crate::api::list::list_models;

#[cfg(not(feature = "async"))]
pub use crate::api::client::Client;

#[cfg(not(feature = "async"))]
pub use crate::api::list::list_models;
