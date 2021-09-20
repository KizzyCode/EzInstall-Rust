#[macro_use] mod run_verbose;
mod list;
mod install;
mod uninstall;
mod delete_cached;

use crate::error::Result;


/// A CLI command interface
pub enum Command {
    /// Lists the installed archives
    List,
    /// Installs an archive from an URL
    Install { url: String },
    /// Uninstalls an archive
    Uninstall { url: String },
    /// Deletes a cached archive
    DeleteCached { name: String }
}
impl Command {
    /// Loads the command from the CLI arguments
    pub fn load<V, I, IA>(verb: V, args: I) -> Result<Self>
        where V: AsRef<str>, I: IntoIterator<Item = IA>, IA: ToString
    {
        match verb.as_ref() {
            "list" => {
                let _args = Self::words_get_n(args, 0)?;
                Ok(Self::List)
            },
            "install" => {
                let mut args = Self::words_get_n(args, 1)?;
                Ok(Self::Install { url: args.remove(0) })
            },
            "uninstall" => {
                let mut args = Self::words_get_n(args, 1)?;
                Ok(Self::Uninstall { url: args.remove(0) })
            },
            "delete-cached" => {
                let mut args = Self::words_get_n(args, 1)?;
                Ok(Self::DeleteCached { name: args.remove(0) })
            },
            verb => Err(einval!("Invalid verb: {}", verb))
        }
    }

    /// Executes the command
    pub fn exec(self) -> Result {
        match self {
            Self::List => list::exec(),
            Self::Install { url } => install::exec(url),
            Self::Uninstall { url } => uninstall::exec(url),
            Self::DeleteCached { name } => delete_cached::exec(name)
        }
    }

    /// Gets exactly `n` words or raises an error if the iterator contains less or more words
    fn words_get_n<I, IA>(words: I, n: usize) -> Result<Vec<String>> where I: IntoIterator<Item = IA>, IA: ToString {
        let words: Vec<_> = words.into_iter().take(n).map(|w| w.to_string()).collect();
        match words.len() {
            len if len == n => Ok(words),
            len => Err(einval!("Invalid amounts of arguments for the given verb (expected {}; got {})", n, len))
        }
    }
}
