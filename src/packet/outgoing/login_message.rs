use crate::util::writer::Writer;

use super::{LoginMessage, OutPacketBuildable};

impl OutPacketBuildable for LoginMessage {
    fn try_build(&self, writer: &mut Writer) -> anyhow::Result<()> {
        todo!()
    }
}