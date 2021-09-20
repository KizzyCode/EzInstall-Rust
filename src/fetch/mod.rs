mod fetch_impl;
mod verify;

use crate::{
    error::Result,
    fetch::{ 
        verify::Verify,
        fetch_impl::{ Fetch, FetchImpl }
    }
};
use std::{ fs, path::Path };
    

/// A remote resource
pub struct RemoteResource {
    /// The URL of the resource
    url: String,
    /// The fragment/hash part of the URL
    hash: Option<String>
}
impl RemoteResource {
    /// The valid chars
    const VALID_CHARS: &'static [char] = &[
        'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
        'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
        '0','1','2','3','4','5','6','7','8','9',
        '_','-'
    ];

    /// Creates a new remote resource referenced by the given URL
    pub fn new<T>(url: T) -> Result<Self> where T: AsRef<str> {
        // Parse the URL
        let url = url.as_ref();
        let (url, hash) = url.split_once("#")
            .map(|(url, hash)| (url.to_string(), Some(hash.to_string())))
            .unwrap_or((url.to_string(), None));

        Ok(Self { url, hash })
    }
    
    /// Creates a local name for the resource from the given URL
    pub fn local_name(&self) -> String {
        // Get the fingerprint string
        let fingerprint = self.hash.as_ref()
            .map(|h| h.as_str()).unwrap_or("checksum=ignored");
        
        // Sanitize the archive name
        let sanitized = self.url.chars()
            .take(128)
            .map(|char| {
                match Self::VALID_CHARS.contains(&char) {
                    true => char,
                    false => '_'
                }
            })
            .collect::<String>();
        
        format!("{}.{}.tar.gz", sanitized, fingerprint)
    }

    /// Fetches the file and saves it to the given path
    pub fn fetch<T>(&self, path: T) -> Result where T: AsRef<Path> {
        // Fetch the data
        let fetch = FetchImpl::new()?;
        let data = fetch.get(&self.url)?;
        
        // Save the file to disk
        let _hash_ok = Verify::check(&data, self.hash.as_ref())?;
        fs::write(path, data)?;
        Ok(())
    }
}
