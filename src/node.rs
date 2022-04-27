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

use crate::cli::{Args, FromStr, Result, Subcommand, Verbosity};
use derive_more::Display;
use std::error::Error;

/// Node CLI
#[derive(Args, Clone, Debug)]
pub struct Arguments {
    /// Runtime Command
    #[clap(subcommand)]
    pub command: Command,
}

/// Node Command
#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Start a node instance
    Start {
        ///
        #[clap(long)]
        runtime: Runtime,
    },
}

/// Runtime Kind
#[derive(Clone, Debug)]
pub enum Runtime {
    /// Manta Runtime
    Manta,

    /// Calamari Runtime
    Calamari,

    /// Dolphin Runtime
    Dolphin,
}

impl FromStr for Runtime {
    type Err = ParseRuntimeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "manta" => Self::Manta,
            "calamari" => Self::Calamari,
            "dolphin" => Self::Dolphin,
            s => return Err(ParseRuntimeError(s.to_owned())),
        })
    }
}

///
#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub struct ParseRuntimeError(pub String);

impl Error for ParseRuntimeError {}

///
#[inline]
pub fn run(args: Arguments, verbose: Verbosity) -> Result {
    match args.command {
        Command::Start { runtime } => match runtime {
            Runtime::Manta => todo!(),
            Runtime::Calamari => todo!(),
            Runtime::Dolphin => todo!(),
        },
    }
}
