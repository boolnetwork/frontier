// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2015-2022 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::collections::BTreeMap;

use ethereum::AccessListItem;
use ethereum_types::{H160, H256, U256};
use serde::Deserialize;
use jsonrpsee::types::error::CallError;

use crate::types::Bytes;

/// Call request
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CallRequest {
	/// From
	pub from: Option<H160>,
	/// To
	pub to: Option<H160>,
	/// Gas Price
	pub gas_price: Option<U256>,
	/// EIP-1559 Max base fee the caller is willing to pay
	pub max_fee_per_gas: Option<U256>,
	/// EIP-1559 Priority fee the caller is paying to the block author
	pub max_priority_fee_per_gas: Option<U256>,
	/// Gas
	pub gas: Option<U256>,
	/// Value
	pub value: Option<U256>,
	/// Data
	#[serde(default, flatten)]
    pub input: CallInput,
	/// Nonce
	pub nonce: Option<U256>,
	/// AccessList
	pub access_list: Option<Vec<AccessListItem>>,
	/// EIP-2718 type
	#[serde(rename = "type")]
	pub transaction_type: Option<U256>,
}

// State override
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CallStateOverride {
	/// Fake balance to set for the account before executing the call.
	pub balance: Option<U256>,
	/// Fake nonce to set for the account before executing the call.
	pub nonce: Option<U256>,
	/// Fake EVM bytecode to inject into the account before executing the call.
	pub code: Option<Bytes>,
	/// Fake key-value mapping to override all slots in the account storage before
	/// executing the call.
	pub state: Option<BTreeMap<H256, H256>>,
	/// Fake key-value mapping to override individual slots in the account storage before
	/// executing the call.
	pub state_diff: Option<BTreeMap<H256, H256>>,
}


#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize)]
pub struct CallInput {
    /// Transaction data
    pub input: Option<Bytes>,
    /// Transaction data
    ///
    /// This is the same as `input` but is used for backwards compatibility: <https://github.com/ethereum/go-ethereum/issues/15628>
    pub data: Option<Bytes>,
}

impl CallInput {
    /// Consumes the type and returns the optional input data.
    ///
    /// Returns an error if both `data` and `input` fields are set and not equal.
    pub fn try_into_unique_input(self) -> Result<Option<Bytes>, CallError> {
        let Self { input, data } = self;
        match (input, data) {
            (Some(input), Some(data)) if input == data => Ok(Some(input)),
            (Some(_), Some(_)) => Err(CallError::InvalidParams(CallInputError::default().into())),
            (Some(input), None) => Ok(Some(input)),
            (None, Some(data)) => Ok(Some(data)),
            (None, None) => Ok(None),
        }
    }

    /// Consumes the type and returns the optional input data.
    ///
    /// Returns an error if both `data` and `input` fields are set and not equal.
    pub fn unique_input(&self) -> Result<Option<&Bytes>, CallError> {
        let Self { input, data } = self;
        match (input, data) {
            (Some(input), Some(data)) if input == data => Ok(Some(input)),
            (Some(_), Some(_)) => Err(CallError::InvalidParams(CallInputError::default().into())),
            (Some(input), None) => Ok(Some(input)),
            (None, Some(data)) => Ok(Some(data)),
            (None, None) => Ok(None),
        }
    }
}

impl From<Bytes> for CallInput {
    fn from(input: Bytes) -> Self {
        Self { input: Some(input), data: None }
    }
}

impl From<Option<Bytes>> for CallInput {
    fn from(input: Option<Bytes>) -> Self {
        Self { input, data: None }
    }
}

#[derive(Debug, Default, thiserror::Error)]
#[error("both \"data\" and \"input\" are set and not equal. Please use \"input\" to pass transaction call data")]
#[non_exhaustive]
pub struct CallInputError;

