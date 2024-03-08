use crate::util::writer::Writer;

use super::{ OutPacketBuildable, SetEncData};

impl OutPacketBuildable for SetEncData {
    fn try_build(&self, writer: &mut Writer) -> anyhow::Result<()> {
        todo!()
    }
}