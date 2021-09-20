/// A fetch implementation
#[cfg_attr(target_os = "freebsd", path = "fetch_impl_freebsdfetch.rs")]
#[cfg_attr(target_os = "macos", path = "fetch_impl_curl.rs")]
#[cfg_attr(target_os = "linux", path = "fetch_impl_curl.rs")]
mod fetch_impl_real;

use crate::error::Result;

// Re-export the platform specific implementation
pub use fetch_impl_real::FetchImpl;


/// A fetch implementation
pub trait Fetch where Self: Sized {
    /// Creates the platform specific fetch implementation
    fn new() -> Result<Self>;

    /// Gets some data from the given `url`
    fn get<T>(&self, url: T) -> Result<Vec<u8>> where T: AsRef<str>;
}


/// Unit tests for the fetch implementation
#[cfg(test)]
mod tests {
    use crate::fetch::{ Fetch, fetch_impl::FetchImpl };

    #[test]
    fn test_fetch_get() {
        let data = FetchImpl::new().expect("Failed to initialize fetch instance")
            .get("https://example.com").expect("Failed to get resource");
        assert!(!data.is_empty());
    }
}