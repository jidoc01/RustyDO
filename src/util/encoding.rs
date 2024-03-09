use encoding::{all::WINDOWS_949, codec::korean::Windows949Encoding, Encoding};

/// DO uses CP949 only.
const ENCODING: &Windows949Encoding = WINDOWS_949;

pub fn encode_to_vec<T: AsRef<str>>(s: T) -> anyhow::Result<Vec<u8>> {
    let input = s.as_ref();
    let trap = encoding::EncoderTrap::Strict;
    match ENCODING.encode(input, trap) {
        Ok(b) => Ok(b),
        Err(e) => anyhow::bail!("encoding error")
    }
}

pub fn decode_to_string<T: AsRef<[u8]>>(b: T) -> anyhow::Result<String> {
    let input = b.as_ref();
    let trap = encoding::DecoderTrap::Replace;
    match ENCODING.decode(input, trap) {
        Ok(s) => Ok(s),
        Err(e) => anyhow::bail!("decoding error")
    }
}