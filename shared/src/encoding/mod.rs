// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod bytes;
mod cbor;
mod errors;
mod hash;
mod vec;

pub use serde::{de, ser};
pub use serde_bytes;

pub use self::bytes::*;
pub use self::cbor::*;
pub use self::errors::*;
pub use self::hash::*;
pub use self::vec::*;

// TODO: these really don't work all that well in a shared context like this as anyone importing
// them also need to _explicitly_ import the serde_tuple & serde_repr crates. These are _macros_,
// not normal items.

pub mod tuple {
    pub use serde_tuple::{self, Deserialize_tuple, Serialize_tuple};
}

pub mod repr {
    pub use serde_repr::{Deserialize_repr, Serialize_repr};
}

/// Serializes a value to a vector.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ser::Serialize + ?Sized,
{
    let mut vec = Vec::new();
    ciborium::ser::into_writer(value, &mut vec)?;
    Ok(vec)
}

/// Deserialize a value from a slice.
pub fn from_slice<'a, T>(slice: &'a [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    ciborium::de::from_reader(slice).map_err(Into::into)
}
