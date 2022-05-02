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

//! Signer CLI

use crate::{
    cli::{ErrorKind, Parser, ParserExt, Result, Subcommand, Verbosity},
    node::Runtime,
};
use manta_crypto::rand::{CryptoRng, OsRng, RngCore, Sample};
use manta_signer::{
    config::Config,
    secret::{Authorizer, Password, PasswordFuture, SecretString},
    service,
};
use std::path::PathBuf;

/// Builds the default [`Config`] for a Signer.
#[inline]
pub fn build_config() -> Result<Config> {
    match Config::try_default() {
        Some(config) => Ok(config),
        _ => Err(Arguments::error(
            ErrorKind::Io,
            "Unable to build default Signer configuration.",
        )),
    }
}

/// User Authorizer with `STDIN` Password Input
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct User;

impl Authorizer for User {
    #[inline]
    fn password(&mut self) -> PasswordFuture {
        Box::pin(async move {
            match tokio::task::spawn_blocking(|| rpassword::prompt_password("Enter Password: "))
                .await
            {
                Ok(Ok(password)) => Password::from_known(SecretString::new(password)),
                _ => Password::from_unknown(),
            }
        })
    }
}

/// Mock User Authorizer for Temporary Signers
pub struct MockUser(SecretString);

impl MockUser {
    /// Builds a new [`MockUser`] from a random password sampled from `rng`.
    #[inline]
    pub fn new<R>(rng: &mut R) -> Self
    where
        R: CryptoRng + RngCore + ?Sized,
    {
        Self(SecretString::new(u128::gen(rng).to_string()))
    }
}

impl Authorizer for MockUser {
    #[inline]
    fn password(&mut self) -> PasswordFuture {
        Box::pin(async move { Password::from_known(self.0.clone()) })
    }
}

/// Signer Command
#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Starts a Local Signer
    Start {
        /// Specify the runtime to use for this Signer
        #[clap(arg_enum, long)]
        runtime: Runtime,

        /// Path to Signer Data File
        ///
        /// If unset, uses the default known Signer location if it exists.
        data: Option<PathBuf>,

        /// Use a temporary directory for storing the Signer state
        #[clap(long)]
        temp: bool,

        /// Specify a custom serivce URL to use for this Signer
        #[clap(long)]
        url: Option<String>,
    },

    /// Lists all Known Signers
    List,
}

/// Signer CLI
#[derive(Clone, Debug, Parser)]
pub struct Arguments {
    /// Signer Command
    #[clap(subcommand)]
    pub command: Command,
}

impl Arguments {
    /// Runs a signer implementation according to [`Self`].
    #[inline]
    pub fn run(self, verbose: Verbosity) -> Result {
        // TODO: Use the verbosity flag.
        let _ = verbose;
        match self.command {
            Command::Start {
                runtime,
                data,
                temp,
                url,
            } => {
                let mut config = build_config()?;
                match (data, temp) {
                    (Some(path), false) => {
                        config.data_path = path;
                    }
                    (None, true) => {
                        config.data_path = Self::tempdir()?.path().join("storage.dat");
                    }
                    (Some(_), true) => Self::with_error(
                        ErrorKind::ArgumentConflict,
                        "Cannot use the temporary file argument with the data path argument.",
                    )?,
                    _ => {}
                }
                if let Runtime::Manta | Runtime::Calamari = runtime {
                    // FIXME: For now we are not able to switch runtimes on the `service`.
                    Self::with_error(
                        ErrorKind::ValueValidation,
                        "For the current implementation, only dolphin is allowed as the signer runtime.",
                    )?;
                }
                if let Some(service_url) = url {
                    // FIXME: Parse it as a socket address here so that we can validate it before
                    //        passing it to the signer. This requires changes in the `Config` type.
                    config.service_url = service_url;
                }
                // NOTE: We have to use `async_std` here because the `serivce` uses it internally.
                if let Err(err) = async_std::task::block_on(async {
                    if temp {
                        service::start(config, MockUser::new(&mut OsRng)).await
                    } else {
                        service::start(config, User).await
                    }
                }) {
                    Self::with_error(
                        ErrorKind::Io,
                        format_args!("Unable to start signer service: {:?}", err),
                    )?;
                }
            }
            Command::List => {
                let config = build_config()?;
                if config.data_path.exists() {
                    println!("Default Dolphin Signer: {}", config.data_path.display());
                }
            }
        }
        Ok(())
    }
}
