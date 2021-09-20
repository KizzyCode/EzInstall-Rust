use crate::{ error::Result, fetch::fetch_impl::Fetch };
use ezexec::{ ExecBuilder, lookup::Binary };
use std::convert::TryInto;


/// A `curl` based fetch implementation
pub struct FetchImpl;
impl Fetch for FetchImpl {
    fn new() -> Result<Self> {
        let _binary = Binary::find("curl")?;
        Ok(Self)
    }

    fn get<T>(&self, url: T) -> Result<Vec<u8>> where T: AsRef<str> {
        let data = ExecBuilder::with_name("curl", ["--location", "--silent", url.as_ref()])?
            .spawn_captured()?
            .try_into()?;
        Ok(data)
    }
}