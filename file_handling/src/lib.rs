use std::fs;
use std::io;
use std::io::Write;

pub fn create_writable_file(file_path: &str) -> Result<fs::File, io::Error> {
    fs::File::create(file_path)
}

pub fn write_to_file(mut file: &fs::File, contents: &[u8]) -> Result<(), io::Error> {
    file.write(contents)?;
    Ok(())
}

/// Read a text files contents and returns its contents as a String
pub fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_file() {
        create_writable_file("test-create.log").unwrap();
    }

    #[test]
    fn write_file() {
        let mut file = create_writable_file("test-write.log").unwrap();
        write_to_file(&mut file, b"test123").unwrap();
    }

    #[test]
    fn read_file() {
        create_writable_file("test-read.log").unwrap();
        read_file_contents("test-read.log").unwrap();
    }
}
