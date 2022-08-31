// https://ferrous-systems.github.io/teaching-material/assignments/durable-file.html
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct DurableFile {
    file: File,
    // needs_sync: bool,
}

impl  DurableFile {
    pub fn new(file: File) -> DurableFile {
        DurableFile { file: file }
    }
}

impl  Write for DurableFile {
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

impl   Read for DurableFile   {
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
    // use tempdir::TempDir;
    use std::fs::OpenOptions;

    #[test]
    fn write_test() {
        let dir = tempdir::TempDir::new("tests").unwrap();
        {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(dir.path().join("bar.txt"))
                .unwrap();
            let mut durable_file = DurableFile::new(file);
            match durable_file.write(b"hello") {
                Ok(_) => println!(""),
                Err(d) => println!("didn't write file: {d}"),
            };
        }

        let file = OpenOptions::new()
            .read(true)
            .open(dir.path().join("bar.txt"))
            .unwrap();

        let mut durable_file = DurableFile::new(file);

        let mut container_string = [0; 5];
        let read_size = durable_file.read(&mut container_string).unwrap();

        assert_eq!(b"hello", &container_string);
        assert_eq!(5, read_size);
    }

    #[test]
    fn read_test() {
        let file = OpenOptions::new()
            .read(true)
            .open("./foo.txt")
            .unwrap();

        let mut durable_file = DurableFile::new(file);

        let mut container_string = [0; 5];
        let read_size = durable_file.read(&mut container_string).unwrap();

        assert_eq!(b"hello", &container_string);
        assert_eq!(5, read_size)
    }
}
