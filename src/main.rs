#[macro_use] mod error;
mod fetch;
mod tar;
mod packagedir;
mod cli;

use crate::{ cli::Command, error::Result };
use std::{ env, process };


pub fn main() {
    /// The real main function
    fn _main_real() -> Result {
        // Get the args
        let mut args = env::args();
        let _binary = args.next().ok_or(einval!("Missing arguments"))?;
        let verb = args.next().ok_or(einval!("Missing verb"))?;

        // Execute the action
        let command = Command::load(verb, args)?;
        command.exec()
    }

    // Execute the main function
    if let Err(e) = _main_real() {
        eprintln!("Fatal error: {}", e);
        process::exit(1);
    }
}
