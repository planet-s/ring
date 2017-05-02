// Copyright 2015-2017 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! This module exists to make `signature_from_bytes()` public within the crate
//! but private outside of the crate.

use ec;

/// A public key signature returned from a signing operation.
pub struct Signature {
    value: [u8; MAX_LEN],
    len: usize,
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] { &self.value[..self.len] }
}

pub fn signature_from_bytes(bytes: &[u8]) -> Signature {
    let mut r = Signature {
       value: [0; MAX_LEN],
        len: bytes.len(),
    };
    r.value[..bytes.len()].copy_from_slice(bytes);
    r
}

/// The maximum length of a signature that can be stored in a `Signature`.
///
/// In particular, this isn't used for RSA signatures.
const MAX_LEN: usize = 1 + (2 * ec::ELEM_MAX_BYTES);