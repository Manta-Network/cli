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

//! Node CLI

use crate::cli::{ArgEnum, ErrorKind, Parser, ParserExt, Result, Subcommand, Verbosity};

/// Runtime Kind
#[derive(ArgEnum, Clone, Debug)]
pub enum Runtime {
    /// Manta Runtime
    Manta,

    /// Calamari Runtime
    Calamari,

    /// Dolphin Runtime
    Dolphin,
}

/// Node Command
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Start a node instance
    Start {
        /// Manta Node CLI Arguments
        #[clap(flatten)]
        cli: manta::cli::Cli,
    },
}

/// Node CLI
#[derive(Debug, Parser)]
pub struct Arguments {
    /// Runtime Command
    #[clap(subcommand)]
    pub command: Command,
}

impl Arguments {
    /// Runs a node implementation according to [`Self`].
    #[inline]
    pub fn run(self, verbose: Verbosity) -> Result {
        // FIXME: Use the verbosity flag.
        let _ = verbose;
        match self.command {
            Command::Start { cli } => {
                // FIXME: Expose the error API from the node so that we can give the most expressive
                //        error here.
                manta::command::run_with(cli).map_err(|e| Self::error(ErrorKind::Io, e))
            }
        }
    }
}
