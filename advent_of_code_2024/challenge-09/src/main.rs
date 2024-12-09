use std::fs::File;
use std::io::{self, Read};

struct Processor {
    row_input: String,
}

fn main() -> io::Result<()> {

    let file_path = "test.txt";

    let mut processor = Processor{
        row_input: String::new()
    };

    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file content into the String
    file.read_to_string(&mut processor.row_input)?;

    // Print the content (optional)
    println!("File content:\n{}", processor.row_input);

    Ok(())
}
