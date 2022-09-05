// RustyDO
//
// Copyright 2022. JungHyun Kim (jidoc01).
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

mod rc4;

use once_cell::sync::Lazy;

const HEADER_LENGTH: usize = 9;
const SALT_LENGTH: usize = 16 - 5;

static FEED: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut key = b"\xf6\xef\x8b\xa1\x5c".to_vec();
    let mut salt = b"\x00".repeat(SALT_LENGTH);
    key.append(&mut salt);
    key
});

// TODO: Do not create a new buffer.
pub fn transfer(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; data.len()];
    rc4::transfer(data, &mut out, &FEED);
    out
}

#[inline]
pub fn decode(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let len = data.len();
    anyhow::ensure!(len == HEADER_LENGTH);
    let key = data[len - 1];
    anyhow::ensure!(key < 8);
    let out =
        (0 .. len - 1) // Except the last element.
        .map(|i| (data[i] << key) | (data[(i + 1) % 8] >> (8 - key)))
        .collect();
    Ok(out)
}

/// Encode 9 (8 + 1) bytes header.
/// Returns 9 bytes including its seed value.
#[inline]
pub fn encode(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let len = data.len();
    anyhow::ensure!(len == HEADER_LENGTH);
    let key = data[len - 1];
    anyhow::ensure!(key < 8);
    let mut out: Vec<u8> =
        (0 .. len - 1)
        .map(|i| (data[i] >> key) | (data[(i + 8 - 1) % 8] << (8 - key)))
        .collect();
    out.push(key); // Append its seed.
    Ok(out)
}
