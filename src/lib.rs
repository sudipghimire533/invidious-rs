pub mod endpoints;
pub mod endpoints_impl;
pub mod types;
pub mod utils;
#[cfg(test)]
mod tests;

pub mod reexports {
    pub use url;
    pub use url::Url;
}
