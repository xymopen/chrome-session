use std::io::prelude::*;
use std::io::Result;

pub(crate) struct Count<'a> {
    inner: &'a mut dyn Write,
    count: usize,
}

impl<'a> Count<'a> {
    pub(crate) fn count(&mut self) -> usize {
        let count = self.count;
        self.count = 0;
        return count;
    }
}

impl<'a> Write for Count<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.inner.write(buf)?;
        self.count += n;
        return Ok(n);
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

impl<'a> From<&'a mut dyn Write> for Count<'a> {
    fn from(writer: &'a mut dyn Write) -> Self {
        Count {
            inner: writer,
            count: 0,
        }
    }
}
