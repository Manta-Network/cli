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

//! Wallet CLI

use crate::cli::{Args, Result, Subcommand, Verbosity};
use std::path::PathBuf;

/// Wallet CLI
#[derive(Args, Clone, Debug)]
pub struct Arguments {
    /// Wallet Command
    #[clap(subcommand)]
    pub command: Command,
}

/// Wallet Command
#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Generate a new Wallet
    Generate,

    /// Loads an Existing Wallet
    Start {
        /// Path to Wallet Source File
        ///
        /// If unset, uses the default known wallet if it exists.
        source: Option<PathBuf>,
    },

    /// Lists all Known Wallets
    List,
}

///
#[inline]
pub fn run(args: Arguments, verbose: Verbosity) -> Result {
    match args.command {
        Command::Generate => todo!(),
        Command::Start { source } => todo!(),
        Command::List => todo!(),
    }
}
