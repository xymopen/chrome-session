use std::io::prelude::*;
use std::io::Result;

pub(crate) struct Count<'a> {
    inner: &'a mut dyn Read,
    count: usize,
}

impl<'a> Count<'a> {
    pub(crate) fn count(&mut self) -> usize {
        let count = self.count;
        self.count = 0;
        return count;
    }
}

impl<'a> Read for Count<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.inner.read(buf)?;
        self.count += n;
        return Ok(n);
    }
}

impl<'a> From<&'a mut dyn Read> for Count<'a> {
    fn from(reader: &'a mut dyn Read) -> Self {
        Count {
            inner: reader,
            count: 0,
        }
    }
}
