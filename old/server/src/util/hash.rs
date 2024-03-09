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

use sha3::{Sha3_256, Digest};

/// We use SHA-3.
pub fn from_str(s: &str) -> Vec<u8> {
    let mut hasher = Sha3_256::default();
    hasher.update(s);
    hasher
        .finalize()
        .to_vec()
}