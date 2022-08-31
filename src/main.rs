// https://ferrous-systems.github.io/teaching-material/assignments/durable-file.html
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct DurableFile {
    file: File,
    needs_sync: bool,
}

impl  DurableFile {
    pub fn new(file: File) -> DurableFile {
        DurableFile { file: file, needs_sync: false }
    }

    pub fn close(self) {
        let mut df = self;
        let _ = df.flush();
    }
}

impl  Write for DurableFile {
    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.file.sync_all()?;
        self.needs_sync = false;
        self.file.flush()
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.needs_sync = true;
        self.file.write(buf)
    }
}

impl   Read for DurableFile   {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.file.read(buf)
    }
}

impl Drop for DurableFile {
    fn drop(&mut self) {
        if self.needs_sync {
            panic!("file needs sync")
        }
    }
}

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
        let _ = durable_file.flush();

        // rust has a full tantrum about reusing file descriptors
        // to get around this, we just refile'd our file. 
        let refile = OpenOptions::new()
            .read(true)
            .open(dir.path().join("bar.txt"))
            .unwrap();
        let mut durable_file2 = DurableFile::new(refile);

        let mut container_string = [0; 5];
        let read_size = durable_file2.read(&mut container_string).unwrap();

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

    #[test]
    #[should_panic(expected = "file needs sync")]
    fn sync_fail_test() {
        let dir = tempdir::TempDir::new("tests").unwrap();
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
}
