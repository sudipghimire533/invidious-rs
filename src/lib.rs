pub mod endpoints;
pub mod instance;
pub mod types;
pub mod utils;
pub mod web_client;
// #[cfg(feature = "reqwest_impl")]
pub mod reqwest_impl;

pub mod reexports {
    pub use url;
    pub use url::Url;
}
