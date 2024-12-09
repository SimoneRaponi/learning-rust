use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {

    let file_path = "test.txt";

    // Open the file
    let mut file = File::open(file_path)?;

    // Create a String to store the file content
    let mut content = String::new();

    // Read the file content into the String
    file.read_to_string(&mut content)?;

    // Print the content (optional)
    println!("File content:\n{}", content);

    Ok(())
}
