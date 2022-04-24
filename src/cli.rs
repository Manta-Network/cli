// Copyright 2019-2022 Manta Network.
// This file is part of manta-cli.
//
// manta-cli is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// manta-cli is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with manta-cli.  If not, see <http://www.gnu.org/licenses/>.

//! Manta Command Line Interface

pub use clap::{Args, Error, Parser, Subcommand};
pub use clap_verbosity_flag::Verbosity;

/// CLI Result Type
///
/// Uses `()` as the default type on the `Ok` branch and [`Error`] as the default type on the `Err`
/// branch.
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

/// Manta CLI
#[derive(Clone, Debug, Parser)]
#[clap(about, author, version, long_about = None, propagate_version = true)]
pub struct Arguments {
    /// Command
    #[clap(subcommand)]
    command: Command,

    /// Verbosity
    #[clap(flatten)]
    verbose: Verbosity,
}

/// Defines commands for the [`run`] function.
macro_rules! define_commands {
    ($(($doc:expr, $name:ident, $path:tt)),*$(,)?) => {
        /// Manta CLI Sub-Command
        #[derive(Clone, Debug, Subcommand)]
        pub enum Command {
            $(
                #[doc = $doc]
                $name(crate::$path::Arguments)
            ),*
        }

        /// Runs the CLI on the arguments provided by the command line.
        #[inline]
        pub fn run() -> Result<()> {
            let args = Arguments::try_parse()?;
            match args.command {
                $(Command::$name(command) => crate::$path::run(command, args.verbose)),*
            }
        }
    }
}

define_commands! {
    ("Define or Use a Manta Wallet", Wallet, wallet),
}

/// Runs the [`run`] method and then exits on error.
#[inline]
pub fn run_and_exit() {
    if let Err(err) = run() {
        err.exit()
    }
}
