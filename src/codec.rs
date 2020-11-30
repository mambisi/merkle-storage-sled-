// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, BTreeMap};
use std::ops::Range;

use failure::Fail;
use serde::{Deserialize, Serialize};

use crate::hash::Hash;
use sled::IVec;

/// Possible errors for schema
#[derive(Debug, Fail)]
pub enum SchemaError {
    #[fail(display = "Failed to encode value")]
    EncodeError,
    #[fail(display = "Failed to decode value")]
    DecodeError,
}

/// Encode input value to binary format.
pub trait Encoder: Sized {
    /// Try to encode instance into its binary format
    fn encode(&self) -> Result<Vec<u8>, SchemaError>;
}

/// Decode value from binary format.
pub trait Decoder: Sized {
    /// Try to decode message from its binary format
    fn decode(bytes: &[u8]) -> Result<Self, SchemaError>;
}

/// This trait specifies arbitrary binary encoding and decoding methods for types requiring storing in database
pub trait Codec: Encoder + Decoder {}

impl<T> Codec for T where T: Encoder + Decoder {}

impl Encoder for Hash {
    fn encode(&self) -> Result<Vec<u8>, SchemaError> {
        Ok(self.clone())
    }
}

impl Decoder for Hash {
    fn decode(bytes: &[u8]) -> Result<Self, SchemaError> {
        Ok(bytes.to_vec())
    }
}

impl Encoder for String {
    fn encode(&self) -> Result<Vec<u8>, SchemaError> {
        Ok(self.as_bytes().to_vec())
    }
}

impl Decoder for String {
    fn decode(bytes: &[u8]) -> Result<Self, SchemaError> {
        String::from_utf8(bytes.to_vec()).map_err(|_| SchemaError::DecodeError)
    }
}


/// Generate codec (encoder + decoder) for a numeric type
macro_rules! num_codec {
    ($num:ident) => {
        #[allow(dead_code)]
        impl Decoder for $num {
            fn decode(bytes: &[u8]) -> Result<Self, SchemaError> {
                if bytes.len() == std::mem::size_of::<$num>() {
                    let mut num_bytes: [u8; std::mem::size_of::<$num>()] = Default::default();
                    num_bytes.copy_from_slice(&bytes[..]);
                    Ok($num::from_be_bytes(num_bytes))
                } else {
                    Err(SchemaError::DecodeError)
                }
            }
        }
        #[allow(dead_code)]
        impl Encoder for $num {
            fn encode(&self) -> Result<Vec<u8>, SchemaError> {
                let mut value = Vec::with_capacity(std::mem::size_of::<$num>());
                value.extend(&self.to_be_bytes());
                Ok(value)
            }
        }
    }
}

num_codec!(u8);
num_codec!(i16);
num_codec!(u16);
num_codec!(i64);
num_codec!(u64);
num_codec!(i32);
num_codec!(u32);
num_codec!(usize);

pub trait BincodeEncoded: Sized + Serialize + for<'a> Deserialize<'a> {
    fn decode(bytes: &[u8]) -> Result<Self, SchemaError> {
        bincode::deserialize(bytes)
            .map_err(|_| SchemaError::DecodeError)
    }

    fn encode(&self) -> Result<Vec<u8>, SchemaError> {
        bincode::serialize::<Self>(self)
            .map_err(|_| SchemaError::EncodeError)
    }
}

impl<T> Encoder for T where T: BincodeEncoded {
    fn encode(&self) -> Result<Vec<u8>, SchemaError> {
        T::encode(self)
    }
}

impl<T> Decoder for T where T: BincodeEncoded {
    fn decode(bytes: &[u8]) -> Result<Self, SchemaError> {
        T::decode(bytes)
    }
}

impl<K, V> BincodeEncoded for HashMap<K, V>
    where K: std::hash::Hash + Eq + Serialize + for<'a> Deserialize<'a>,
          V: Serialize + for<'a> Deserialize<'a>
{}

impl<K, V> BincodeEncoded for BTreeMap<K, V>
    where K: Ord + Eq + Serialize + for<'a> Deserialize<'a>,
          V: Serialize + for<'a> Deserialize<'a>
{}

impl BincodeEncoded for () {}

#[macro_export(local_inner_macros)]
macro_rules! num_from_slice {
    ($buf:expr, $from_idx:expr, $num:ident) => {{
        let mut bytes: [u8; std::mem::size_of::<$num>()] = Default::default();
        bytes.copy_from_slice(&$buf[$from_idx .. $from_idx + std::mem::size_of::<$num>()]);
        $num::from_be_bytes(bytes)
    }}
}

#[inline]
pub fn vec_from_slice(buf: &[u8], from_idx: usize, size: usize) -> Vec<u8> {
    buf[from_idx..from_idx + size].to_vec()
}

#[inline]
pub const fn range_from_idx_len(idx: usize, len: usize) -> Range<usize> {
    idx..idx + len
}