use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

pub fn create_writable_file(file_path: &str) -> Result<fs::File, io::Error> {
    fs::File::create(file_path)
}

pub fn open_file_or_create(file_path: &str) -> Result<fs::File, io::Error> {
    OpenOptions::new().create(true).append(true).open(file_path)
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
    fn create_file() -> Result<(), io::Error> {
        create_writable_file("test_create_file.log")?;
        Ok(())
    }

    #[test]
    fn write_file() -> Result<(), io::Error> {
        let mut file = create_writable_file("test_write_file.log")?;
        write_to_file(
            &mut file,
            b"This should create and write to a new file or overwrite an existing files contents",
        )?;
        Ok(())
    }

    #[test]
    fn read_file() -> Result<(), io::Error> {
        create_writable_file("test_read_file.log")?;
        read_file_contents("test_read_file.log")?;
        Ok(())
    }

    #[test]
    fn open_or_create_file_and_write() -> Result<(), io::Error> {
        let mut file = open_file_or_create("test_open_or_create_file_and_write.log").unwrap();
        file.write(b"This should append text\n")?;
        Ok(())
    }
}
