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

//! Simulation CLI

use crate::cli::{ErrorKind, Parser, ParserExt, Result, TempDir, Verbosity};
use manta_crypto::rand::OsRng;
use manta_pay::{
    config::{
        MultiProvingContext, MultiVerifyingContext, NoteEncryptionScheme, Parameters,
        ProvingContext, UtxoAccumulatorModel, UtxoCommitmentScheme, VerifyingContext,
        VoidNumberCommitmentScheme,
    },
    simulation::Simulation,
};
use manta_util::codec::{Decode, IoReader};
use std::fs::File;

/// Simulation CLI
#[derive(Clone, Debug, Parser)]
pub struct Arguments {
    /// Simulation Parameters
    #[clap(flatten)]
    pub simulation: Simulation,
}

impl Arguments {
    /// Runs a private-payment simulation according to [`Self`].
    #[inline]
    pub fn run(self, verbose: Verbosity) -> Result {
        // FIXME: Use the verbosity flag.
        let _ = verbose;
        let (proving_context, verifying_context, parameters, utxo_accumulator_model) =
            match manta_parameters::load_parameters(Self::tempdir()?) {
                Ok(data) => data,
                Err(err) => {
                    return Self::with_error(
                        ErrorKind::Io,
                        format_args!("Unable to fetch SDK parameters: {:?}", err),
                    );
                }
            };
        match tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .build()
        {
            Ok(runtime) => runtime.block_on(async {
                self.simulation
                    .run(
                        &parameters,
                        &utxo_accumulator_model,
                        &proving_context,
                        verifying_context,
                        &mut OsRng,
                    )
                    .await
            }),
            Err(err) => Self::with_error(
                ErrorKind::Io,
                format_args!("Unable to start tokio runtime: {:?}", err),
            )?,
        }
        Ok(())
    }
}
