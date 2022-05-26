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

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![forbid(rustdoc::broken_intra_doc_links)]
#![forbid(missing_docs)]

extern crate alloc;
extern crate derive_more;

pub mod cli;

#[cfg(feature = "node")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "node")))]
pub mod node;

#[cfg(feature = "signer")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "signer")))]
pub mod signer;

#[cfg(feature = "sim")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "sim")))]
pub mod sim;
