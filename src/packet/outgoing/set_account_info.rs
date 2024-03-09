use crate::util::writer::Writer;

use super::{AccountInfo, OutPacketBuildable};

impl OutPacketBuildable for AccountInfo {
    fn opcode (&self) -> u8 {
        todo!()
    }
    fn try_build(&self, writer: &mut Writer) -> anyhow::Result<()> {
        todo!()
    }
}