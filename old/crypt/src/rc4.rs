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

pub fn transfer(src: &[u8], dst: &mut [u8], key: &[u8]) {
    let mut s = generate_state(key);
    do_transfer(src, dst, &mut s);
}

fn generate_state(key: &[u8]) -> Vec<usize> {
    let mut s: Vec<usize> = (0..256).map(|i| i).collect();
    (0..256).fold(0, |j, i| {
        let k = key[i % key.len()] as usize;
        let j = (j + k + s[i]) % 256;
        (s[i], s[j]) = (s[j], s[i]);
        j
    });
    s
}

fn do_transfer(src: &[u8], dst: &mut [u8], s: &mut [usize]) {
    let len = src.len();
    (0..len).fold((0, 0), |(i, j), k| {
        let i = (i + 1) % 256;
        let j = (j + s[i]) % 256;
        (s[i], s[j]) = (s[j], s[i]);
        let v = s[(s[i] + s[j]) % 256] as u8;
        dst[k] = src[k] ^ v;
        (i, j)
    });
}