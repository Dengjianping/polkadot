// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//

//! Error handling related code and Error/Result definitions.

use thiserror::Error;


use polkadot_node_subsystem_util::{Fault, runtime};
use polkadot_subsystem::SubsystemError;


#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(pub Fault<NonFatal, Fatal>);

impl From<NonFatal> for Error {
	fn from(e: NonFatal) -> Self {
		Self(Fault::from_non_fatal(e))
	}
}

impl From<Fatal> for Error {
	fn from(f: Fatal) -> Self {
		Self(Fault::from_fatal(f))
	}
}

impl From<runtime::Error> for Error {
	fn from(o: runtime::Error) -> Self {
		Self(Fault::from_other(o))
	}
}

/// Fatal errors of this subsystem.
#[derive(Debug, Error)]
pub enum Fatal {
	/// Spawning a running task failed.
	#[error("Spawning subsystem task failed")]
	SpawnTask(#[source] SubsystemError),

	/// Errors coming from runtime::Runtime.
	#[error("Error while accessing runtime information")]
	Runtime(#[from] #[source] runtime::Fatal),
}

/// Non-fatal errors of this subsystem.
#[derive(Debug, Error)]
pub enum NonFatal {
	/// We need available active heads for finding relevant authorities.
	#[error("No active heads available - needed for finding relevant authorities.")]
	NoActiveHeads,

	/// This error likely indicates a bug in the coordinator.
	#[error("Oneshot for asking dispute coordinator for active disputes got canceled.")]
	AskActiveDisputesCanceled,

	/// Errors coming from runtime::Runtime.
	#[error("Error while accessing runtime information")]
	Runtime(#[from] #[source] runtime::NonFatal),
}

pub type Result<T> = std::result::Result<T, Error>;