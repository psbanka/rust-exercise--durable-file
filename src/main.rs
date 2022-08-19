use std::fs::File;
use std::io::{Read, Write};

pub struct DurableFile {
    file: File,
    // needs_sync: bool,
}

impl DurableFile {
    pub fn new(file: File) -> DurableFile {
        DurableFile { file: file }
    }
}

impl Write for DurableFile {
    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.file.flush()
    }
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        dbg!(buf);
        self.file.write(buf)
    }
}

impl Read for DurableFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.file.read(buf)
    }
}

// impl From for DurableFile {
// }

fn main() {}

#[cfg(test)]
mod tests {
    use crate::DurableFile;
    use std::io::{Read, Write};
    use tempdir::TempDir;

    #[test]
    fn write_test() {
        let dir = TempDir::new("tests").unwrap();
        let file = std::fs::File::create(dir.path().join("foo.txt")).unwrap();
        let mut durable_file = DurableFile::new(file);
        durable_file.write(b"hello").unwrap();
        let container_string = &mut [u8];
        durable_file.read(container_string).unwrap();
        expect_eq!(container_string, "hello");
        // Look how smart we are: if we use file here, it's gonna get mad.
        // file.write(b"hello world").unwrap();
    }
}
