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

//! Manta Command Line Interface Utilities

use core::future::Future;
use std::io;
use tokio::runtime::{Builder, Runtime};

/// Creates a default [`Runtime`] and then runs [`block_on`](Runtime::block_on).
#[inline]
pub fn block_on<F>(future: F) -> io::Result<F::Output>
where
    F: Future,
{
    Ok(Runtime::new()?.block_on(future))
}

/// Uses the `builder` to create a [`Runtime`] and then runs [`block_on`](Runtime::block_on).
#[inline]
pub fn build_and_block_on<F>(builder: &mut Builder, future: F) -> io::Result<F::Output>
where
    F: Future,
{
    Ok(builder.build()?.block_on(future))
}
