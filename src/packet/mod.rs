use crate::{encrypt::{encrypt_body, encrypt_header}, Writer, BODY_MAGIC_STAMP, TAIL_SIZE};

use self::outgoing::OutPacketBuildable;

pub mod incoming;
pub mod outgoing;
pub mod packet_receiver;



pub fn build_packet(pkt: &dyn OutPacketBuildable) -> anyhow::Result<Vec<u8>> {
    let mut out = Vec::new();
    let body = build_body(pkt)?;
    let mut w = Writer::from_vec_mut(&mut out);
    build_head(&mut w, body.len())?;
    w.write_bytes(body)?;
    build_tail(&mut w)?;
    Ok(out)
}

fn build_head(w: &mut Writer, body_len: usize) -> anyhow::Result<()> {
    let crypto_seed = rand::random::<u8>() % 7 + 1; // 1 ~ 7
    w.write_u16(body_len as u16)?;
    w.write_u16(0xb9)?;
    w.write_u16(0x08)?;
    w.write_u16(0x09)?;
    w.write_u8(crypto_seed)?;
    encrypt_header(w.get_mut());
    Ok(())
}

fn build_body(pkt: &dyn OutPacketBuildable) -> anyhow::Result<Vec<u8>> {
    let mut v = Vec::new();
    let mut writer = Writer::from_vec_mut(&mut v);
    writer.write_u8(pkt.opcode())?;
    writer.write_u8(0)?;
    writer.write_u16(0)?;
    writer.write_u32(BODY_MAGIC_STAMP)?;
    pkt.try_build(&mut writer)?;
    encrypt_body(&mut v);
    Ok(v)
}

fn build_tail(w: &mut Writer) -> anyhow::Result<()> {
    w.write_bytes(vec![0u8; TAIL_SIZE])?;
    Ok(())
}