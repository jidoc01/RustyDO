use crate::util::writer::Writer;

use super::{OutPacketBuildable, SetAccountInfo};

impl OutPacketBuildable for SetAccountInfo {
    fn try_build(&self, writer: &mut Writer) -> anyhow::Result<()> {
        todo!()
    }
}