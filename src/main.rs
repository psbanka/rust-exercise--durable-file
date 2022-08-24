use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct DurableFile<'a> {
    file: &'a mut File,
    // needs_sync: bool,
}

impl <'a> DurableFile<'a> {
    pub fn new(file: &'a mut File) -> DurableFile<'a> {
        DurableFile { file: file }
    }
}

impl <'a> Write for DurableFile<'a> {
    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.file.sync_all()?;
        self.file.flush()
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        let stuff = self.file.write(buf)?;
        dbg!("write");
        dbg!(&stuff);

        dbg!("test");
        dbg!(&self.file);

        let mut read_buf = [0; 5];
        let stuff2 = self.file.read(&mut read_buf);
        dbg!("read");
        dbg!(&self.file);
        dbg!(&stuff2);
        Ok(stuff)
    }
}

impl <'a>  Read for DurableFile <'a>  {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let stuff = self.file.read(buf)?;
        // dbg!("read");
        // dbg!(&stuff);
        Ok(stuff)
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
    use std::fs::OpenOptions;

    #[test]
    fn write_test() {
        let dir = TempDir::new("tests").unwrap();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(dir.path().join("foo.txt"))
            .unwrap();
        let mut durable_file = DurableFile::new(&mut file);
        match durable_file.write(b"hello") {
            Ok(_) => println!(""),
            Err(d) => println!("didn't write file: {d}"),
        };

        let mut container_string = [0; 5];
        match durable_file.read(&mut container_string) {
            Ok(y) => println!("Success!{y}"),
            Err(d) => println!("failed to read file: {d}"),
        };
        dbg!(&container_string);
        assert_eq!(&container_string, b"hello");
        // Look how smart we are: if we use file here, it's gonna get mad.
        // file.write(b"hello world").unwrap();
    }
}
