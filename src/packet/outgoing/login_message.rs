use crate::util::writer::Writer;

use super::{LoginMessage, OutPacketBuildable};

impl OutPacketBuildable for LoginMessage {
    fn opcode(&self) -> u8 { 0 }
    fn try_build(&self, writer: &mut Writer) -> anyhow::Result<()> {
        let kind = self.0.clone();
        writer.write_u32(kind as u32)?;
        Ok(())
    }
}