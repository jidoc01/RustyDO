use super::encoding::decode_to_string;

pub struct Reader<'a> {
    data: &'a [u8],
    off: usize,
}

impl<'a> Reader<'a> {
    pub fn from_ref(data: &'a [u8]) -> Self {
        Self {
            data,
            off: 0,
        }
    }
}

impl Reader<'_> {
    fn head(&self) -> &[u8] {
        &self.data
    }

    fn assert_space(&self, n: usize) -> anyhow::Result<()> {
        let total_len = self.data.len();
        let off = self.off;
        let remaining = total_len - off;
        if remaining >= n { Ok (()) }
        else { anyhow::bail!("not enough space") }
    }

    pub fn advance(&mut self, n: usize) {
        self.off += n;
    }

    pub fn read_u8(&mut self) -> anyhow::Result<u8> {
        let head = self.head();
        self.assert_space(1)?;
        let off = self.off;
        let ret = head[off];
        self.advance(1);
        Ok(ret)
    }

    pub fn read_u16(&mut self) -> anyhow::Result<u16> {
        let head = self.head();
        self.assert_space(2)?;
        let off = self.off;
        let ret =
            ((head[off+0] as u16) << 0) | ((head[off+1] as u16) << 8);
        self.advance(2);
        Ok(ret)
    }

    pub fn read_u32(&mut self) -> anyhow::Result<u32> {
        let head = self.head();
        self.assert_space(4)?;
        let off = self.off;
        let ret =
            ((head[off+0] as u32) << 0)  | ((head[off+1] as u32) << 8) |
            ((head[off+2] as u32) << 16) | ((head[off+3] as u32) << 24);
        self.advance(4);
        Ok(ret)
    }

    pub fn read_bytes(&mut self, n: usize) -> anyhow::Result<Vec<u8>> {
        let head = self.head();
        self.assert_space(n)?;
        let off = self.off;
        let ret = head[off..off+n].to_vec();
        self.advance(n);
        Ok(ret)
    }

    pub fn read_fixed_string(&mut self, len: usize) -> anyhow::Result<String> {
        let data = self.read_bytes(len)?;
        let ret = decode_to_string(data)?;
        Ok(ret)
    }
}
