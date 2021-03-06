// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Open Ethereum.

// Open Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Open Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Open Ethereum.  If not, see <http://www.gnu.org/licenses/>.

//! Account state
//! This crate contains code used to create, convert, and update Accounts and the code and storage
//! associated with it. It also defines the trait used to construct a backend to build a complete
//! caching state database.
//! Note: the code that needs access to `ethcore` types such as `Machine` and `Executive` is found in
//! the `executive_state` module in `ethcore`. Most tests for the `State` module in this crate are
//! also found in `executive_state` (for the same reason).

pub mod account;
pub mod backend;
pub mod state;

use ethereum_types::{Address, H256};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Target {
	Balance(Address),
	// Account(Address),
	// CodeHash(Address),
	Storage(Address, H256),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum AccessMode {
	// Create,
	Read,
	// Update,
	// Delete,
	Write,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Access {
	target: Target,
	mode: AccessMode,
}

impl std::fmt::Debug for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.mode {
			AccessMode::Read => f.write_str("R")?,
			AccessMode::Write => f.write_str("W")?,
		}

		match self.target {
			Target::Balance(addr) => f.write_fmt(format_args!("B({:?})", addr))?,
			Target::Storage(addr, entry) => f.write_fmt(format_args!("S({:?}; {:?})", addr, entry))?,
		}

		Ok(())
    }
}

pub use {
	account::Account,
	backend::Backend,
	state::{State, CleanupMode},
};
