use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};

/// Purpose: Gets the file path from the user via command line input.
///
/// Returns: A result containing the file path with type String or an io::Error
///
/// Errors:
/// io::Error
pub fn get_file_path() -> Result<String, io::Error> {
    println!("Enter the file path of an existing task file, or the file path of where you would like one created and stored: ");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;
    Ok(file_path.trim().to_string())
}

/// Purpose: Opens file from the given file path, or creates the file if not found.
///
/// Returns: A result containing an appendable File type, or an io::Error
///
/// Errors:
/// io::Error
pub fn open_file_or_create(file_path: &str) -> Result<fs::File, io::Error> {
    OpenOptions::new().create(true).append(true).open(file_path)
}

/// Purpose: Writes to a File, in the specific usecase of this project, this function is only being used to write to
/// files opened in append mode, if a file given that was not opened in append mode will be overwritten
///
/// Returns: A result containing a unit type or an io::Error
///
/// Errors:
/// io::Error
pub fn write_to_file(mut file: &fs::File, contents: &[u8]) -> Result<(), io::Error> {
    file.write(contents)?;
    Ok(())
}

pub fn create_writable_file(file_path: &str) -> Result<fs::File, io::Error> {
    fs::File::create(file_path)
}

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

    #[test]
    fn file_path() -> Result<(), io::Error> {
        todo!("Look into passing reading and writing buffers to the get_file_path function instead of the function \
        itself handling input, this will allow better unit tests");
    }
}
