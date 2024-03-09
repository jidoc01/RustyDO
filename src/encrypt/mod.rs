mod rc4;

use crate::*;

use crate::constants::HEADER_SIZE;

const BODY_CRYPTO_SIZE: usize = 16;

lazy_static::lazy_static! {
    static ref BODY_CRYPTO_KEY: Vec<u8> = {
        let mut key = b"\xf6\xef\x8b\xa1\x5c".to_vec();
        let mut salt = b"\x00".repeat(BODY_CRYPTO_SIZE - key.len());
        key.append(&mut salt);
        key
    };
}

#[inline]
pub fn encrypt_body<T: AsMut<[u8]>>(mut body: T) {
    rc4::transfer(&mut body, BODY_CRYPTO_KEY.as_slice());
}

#[inline]
pub fn encrypt_header<T: AsMut<[u8]>>(mut header: T) {
    let header = header.as_mut();

    let len = header.len();
    if len != HEADER_SIZE {
        panic!("Invalid header size: {}", len);
    }

    let key = header[HEADER_SIZE - 1];
    assert!(key < 8, "Invalid key: {}", key);

    (0 .. len - 1)
        .into_par_iter()
        .map(|i| {
            (header[i] >> key) | (header[(i + 8 - 1) % 8] << (8 - key))
        })
        .enumerate()
        .collect::<Vec<_>>()
        .iter()
        .for_each(|(i, v)| {
            header[*i] = *v;
        });
}

#[inline]
pub fn decrypt_header<T: AsMut<[u8]>>(mut header: T) {
    let header = header.as_mut();

    let len = header.len();
    if len != HEADER_SIZE {
        panic!("Invalid header size: {}", len);
    }

    let key = header[HEADER_SIZE - 1];
    if key >= 8 {
        // TODO: kick the client who is using an invalid key.
        return;
    }

    (0 .. len - 1)
        .map(|i| {
            (header[i] << key) | (header[(i + 1) % 8] >> (8 - key))
        })
        .enumerate()
        .collect::<Vec<_>>()
        .iter()
        .for_each(|(i, v)| {
            header[*i] = *v;
        });
}
