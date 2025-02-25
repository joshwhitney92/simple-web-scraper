

#[derive(Debug, Clone)]
pub struct FileWriter {}

/*
 * Trait declarations
 */

pub trait WriteToFile {
    fn write_text_to_file(input: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait AppendToFile {
    fn append_text_to_file(input: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait CreateFile {
    fn create_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>>;
}


/*
 * Encapsulate hard dependencies
 */

mod private_helpers {
    use std::{
        fs::{File, OpenOptions},
        io::Write,
    };

    pub fn write_text_to_file(
        input: &str,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file =
            File::create(file_path).map_err(|op| format!("Failed to create file - Error: {op}"))?;

        file.write_all(input.as_bytes())
            .map_err(|op| format!("Failed to write to file - Error: {op}"))?;

        Ok(())
    }

    /// This will overwrite the exsting file.
    pub fn create_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        File::create(file_path).map_err(|op| format!("Failed to create file - Error: {op}"))?;

        Ok(())
    }

    pub fn append_text_to_file(
        input: &str,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(file_path)
            .map_err(|op| format!("Could not open file - Error: {op}"))?;

        file.write_all(input.as_bytes())
            .map_err(|op| format!("Failed to write to file - Error: {op}"))?;

        Ok(())
    }
}

/*
 * Trait Implementations
 */

impl WriteToFile for FileWriter {
    fn write_text_to_file(input: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        private_helpers::write_text_to_file(input, file_path)
    }
}

impl CreateFile for FileWriter {
    fn create_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        private_helpers::create_file(file_path)
    }
}

impl AppendToFile for FileWriter {
    fn append_text_to_file(input: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        private_helpers::append_text_to_file(input, file_path)
    }
}
