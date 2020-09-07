use std::io::{Read, Write};

pub struct Trem<R, W> {
    read: R,
    write: W,
}

impl<R: Read, W: Write> Trem<R, W> {
    pub fn new(read: R, write: W) -> Self {
        Self {read, write}
    }

    /// Write a buffer to the output
    pub fn write(&mut self, buf: &[u8]) {
        self.write.write(buf).unwrap();
    }

    /// Write a buffer to the output (maybe in a different colour) and move the cursor back
    pub fn write_hint(&mut self, buf: &[u8]) {
        let chars = self.write.write(buf).unwrap();
        self.write.write(&[0x18, 0x5b, chars as u8]).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
