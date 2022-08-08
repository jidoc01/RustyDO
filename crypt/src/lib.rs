// Copyright 2022 JungHyun Kim
// This file is part of RustyDO.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

const HEADER_LENGTH: usize = 9;

use once_cell::sync::Lazy;
use winapi::{um::wincrypt::{CryptAcquireContextA, CryptDecrypt, CryptEncrypt, HCRYPTKEY, HCRYPTPROV, CRYPT_NEWKEYSET, HCRYPTHASH, PROV_RSA_FULL, CryptCreateHash, CALG_MD5, CryptHashData, MS_DEF_PROV, CryptDeriveKey, CALG_RC4}, _core::ptr::{null}};

const FEED: &[u8] = b"\x01\x86\x59\x23";

static CRYPT: Lazy<Crypt> = Lazy::new(|| Crypt::default());

pub fn decrypt(enc: &[u8]) -> Vec<u8> {
    CRYPT.decrypt(enc)
}

pub fn encrypt(enc: &[u8]) -> Vec<u8> {
    CRYPT.encrypt(enc)
}

pub fn encode(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    CRYPT.encode_header(data)
}

pub fn decode(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    CRYPT.decode_header(data)
}

#[derive(Clone)]
pub struct Crypt {
    crypt_key: HCRYPTKEY,
    crypt_prov: HCRYPTPROV,
    crypt_hash: HCRYPTHASH
}

impl Crypt {
    pub fn default() -> Crypt {
        assert_eq!(FEED.len(), 4);
        let mut crypt_key: HCRYPTKEY = 0usize;
        let mut crypt_prov: HCRYPTPROV = 0usize;
        let mut crypt_hash: HCRYPTHASH = 0usize;
        unsafe {
            if CryptAcquireContextA(
                &mut crypt_prov, null(), MS_DEF_PROV.as_ptr() as *const i8,
                PROV_RSA_FULL, 0) == 0 {
                // Re-try.
                assert_ne!(CryptAcquireContextA(
                    &mut crypt_prov, null(), MS_DEF_PROV.as_ptr() as *const i8,
                    PROV_RSA_FULL, CRYPT_NEWKEYSET), 0);
            }
            assert_ne!(CryptCreateHash(
                crypt_prov, CALG_MD5, 0,
                0, &mut crypt_hash), 0);
            assert_ne!(CryptHashData(
                crypt_hash, FEED.as_ptr(), FEED.len() as u32,
                0), 0);
            assert_ne!(CryptDeriveKey(
                crypt_prov, CALG_RC4, crypt_hash,
                1, &mut crypt_key), 0);
        }
        Crypt {
            crypt_key, crypt_prov, crypt_hash
        }
    }
        
    /// Decode 9 (8 + 1) bytes header.
    /// Returns 8 bytes except a seed value.
    #[inline]
    pub fn decode_header(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
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
    pub fn encode_header(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
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
    
    /// Decrypt the body of a packet.
    #[inline]
    pub fn decrypt(&self, enc: &[u8]) -> Vec<u8> {
        let mut out = Vec::from(enc);
        unsafe {
            let mut len = enc.len() as u32;
            CryptDecrypt(self.crypt_key, 0, 1,
                        0, out.as_mut_ptr(), &mut len);
        }
        out
    }
    
    /// Encrypt the body of a packet.
    #[inline]
    pub fn encrypt(&self, dec: &[u8]) -> Vec<u8> {
        let mut out = Vec::from(dec);
        unsafe {
            let mut len = dec.len() as u32;
            CryptEncrypt(self.crypt_key, 0, 1,
                        0, out.as_mut_ptr(), &mut len, len);
        }
        out
    }
}
