use std::io::{self, Read, Write};
use std::fs::{OpenOptions, File};
use std::path::Path;

// Function to read file knowing the path
pub fn read_file(path: &str) -> Result<String, io::Error> {

    let mut s = String::new();
    // Opening the file and reading it, the ? operator causes to return the Error value
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)

    // Same code as above but more extensive, in this case we used the match expression
    /*
    // open file
    let f = File::open(path);
    // Check if we opened the file correctly
    let mut f = match f {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    // Check if we read correctly the file
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
    */
}

// Function to write a file and truncating the content
pub fn write_file_truncate(path: &str, text: &str) -> Result<(), io::Error> {

    // Check that the file exists, if dont exists we create the file
    match Path::new(path).is_file() {
        // If exists we want to truncate the file and put the text, we need write and truncate permisions
        true => match OpenOptions::new().write(true).truncate(true).open(path) {
            Ok(mut file) => write!(file, "{}", text),
            Err(e) => return Err(e),
        },
        false => write!(File::create(path)?, "{}", text),
    }
}

// Function to write a file appending the content
pub fn write_file_append(path: &str, text: &str) -> Result<(), io::Error> {

    // Check that the file exists, if dont exists we create the file
    match Path::new(path).is_file() {
        // If exists we want to append the text in the file, we need write and append permisions
        true => match OpenOptions::new().write(true).append(true).open(path) {
            Ok(mut file) => write!(file, "{}", text),
            Err(e) => return Err(e),
        },
        false => write!(File::create(path)?, "{}", text),
    }
}