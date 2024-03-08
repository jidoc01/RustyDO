mod rc4;

use crate::constants::HEADER_SIZE;

pub fn encrypt_body() {
    todo!()
}

pub fn decrypt_body<T: AsMut<[u8]>>(mut body: T) {
    todo!()
}

pub fn encrypt_header() {
    todo!()
}

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

    (0 .. len - 1).for_each(|i| {
        header[i] = (header[i] << key) | (header[(i + 1) % 8] >> (8 - key));
    })
}
