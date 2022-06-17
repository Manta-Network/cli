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

/// Loads parameters from the SDK, using `directory` as a temporary directory to store files.
#[inline]
fn load_parameters(
    tempdir: TempDir,
) -> anyhow::Result<(
    MultiProvingContext,
    MultiVerifyingContext,
    Parameters,
    UtxoAccumulatorModel,
)> {
    let directory = tempdir.path();
    let mint_path = directory.join("mint.dat");
    manta_parameters::pay::testnet::proving::Mint::download(&mint_path)?;
    let private_transfer_path = directory.join("private-transfer.dat");
    manta_parameters::pay::testnet::proving::PrivateTransfer::download(&private_transfer_path)?;
    let reclaim_path = directory.join("reclaim.dat");
    manta_parameters::pay::testnet::proving::Reclaim::download(&reclaim_path)?;
    let proving_context = MultiProvingContext {
        mint: ProvingContext::decode(IoReader(File::open(mint_path)?))
            .expect("Unable to decode MINT proving context."),
        private_transfer: ProvingContext::decode(IoReader(File::open(private_transfer_path)?))
            .expect("Unable to decode PRIVATE_TRANSFER proving context."),
        reclaim: ProvingContext::decode(IoReader(File::open(reclaim_path)?))
            .expect("Unable to decode RECLAIM proving context."),
    };
    let verifying_context = MultiVerifyingContext {
        mint: VerifyingContext::decode(
            manta_parameters::pay::testnet::verifying::Mint::get()
                .expect("Checksum did not match."),
        )
        .expect("Unable to decode MINT verifying context."),
        private_transfer: VerifyingContext::decode(
            manta_parameters::pay::testnet::verifying::PrivateTransfer::get()
                .expect("Checksum did not match."),
        )
        .expect("Unable to decode PRIVATE_TRANSFER verifying context."),
        reclaim: VerifyingContext::decode(
            manta_parameters::pay::testnet::verifying::Reclaim::get()
                .expect("Checksum did not match."),
        )
        .expect("Unable to decode RECLAIM verifying context."),
    };
    let parameters = Parameters {
        note_encryption_scheme: NoteEncryptionScheme::decode(
            manta_parameters::pay::testnet::parameters::NoteEncryptionScheme::get()
                .expect("Checksum did not match."),
        )
        .expect("Unable to decode NOTE_ENCRYPTION_SCHEME parameters."),
        utxo_commitment: UtxoCommitmentScheme::decode(
            manta_parameters::pay::testnet::parameters::UtxoCommitmentScheme::get()
                .expect("Checksum did not match."),
        )
        .expect("Unable to decode UTXO_COMMITMENT_SCHEME parameters."),
        void_number_commitment: VoidNumberCommitmentScheme::decode(
            manta_parameters::pay::testnet::parameters::VoidNumberCommitmentScheme::get()
                .expect("Checksum did not match."),
        )
        .expect("Unable to decode VOID_NUMBER_COMMITMENT_SCHEME parameters."),
    };
    Ok((
        proving_context,
        verifying_context,
        parameters,
        UtxoAccumulatorModel::decode(
            manta_parameters::pay::testnet::parameters::UtxoAccumulatorModel::get()
                .expect("Checksum did not match."),
        )
        .expect("Unable to decode UTXO_ACCUMULATOR_MODEL."),
    ))
}

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
            match load_parameters(Self::tempdir()?) {
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
