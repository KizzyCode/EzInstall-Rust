use crate::{ error::Result, fetch::fetch_impl::Fetch };
use ezexec::{ ExecBuilder, lookup::Binary };
use std::convert::TryInto;


/// A FreeBSD-`fetch` based fetch implementation
pub struct FetchImpl;
impl Fetch for FetchImpl {
    fn new() -> Result<Self> {
        let _binary = Binary::find("fetch")?;
        Ok(Self)
    }

    fn get<T>(&self, url: T) -> Result<Vec<u8>> where T: AsRef<str> {
        let data = ExecBuilder::with_name("fetch", ["--quiet", "--output=-", url.as_ref()])?
            .spawn_captured()?
            .try_into()?;
        Ok(data)
    }
}
