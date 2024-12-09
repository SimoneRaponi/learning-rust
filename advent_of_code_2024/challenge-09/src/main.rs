use std::fs::File;
use std::io::{self, Read};

struct Processor {
    row_input: String,
    decompressed_input: String,
}

impl Processor {

    /// Parses the row_input into two vectors: file_lengths and space_lengths
    fn parse_file(&self) -> (Vec<usize>, Vec<usize>) {
        let mut file_lengths = vec![]; // Vector to store lengths of files
        let mut space_lengths = vec![]; // Vector to store lengths of spaces
        let mut current_num = String::new(); // Temporary string to build number as we parse
    
        // Iterate over each character in the row_input string
        for (index, c) in self.row_input.chars().enumerate() {

            // Build the current number (digit by digit)
            current_num.push(c);
             
            // Try to parse the current_num as a number
            if let Ok(num) = current_num.parse::<usize>() {

                // If the index is even, it's a file length; if odd, it's a space length
                if index % 2 == 0 {
                    file_lengths.push(num); // Store file length
                } else {
                    space_lengths.push(num); // Store space length
                }
                // Clear the current_num after successfully parsing
                current_num.clear();
            }
        }
    
        // Return the two vectors: file lengths and space lengths
        (file_lengths, space_lengths) 
    }

    // Decompresses the file based on file_lengths and space_lengths
    fn decompress_file(&mut self) {        
        // Variable to keep track of unique file IDs
        let mut file_id = 0;

        // Call parse_file to get the two vectors of file and space lengths
        let (file_lengths, space_lengths) = self.parse_file();

        // Create iterators for both file_lengths and space_lengths
        let mut file_iter = file_lengths.iter();
        let mut space_iter = space_lengths.iter();

        // Iterate over both file_sizes and space_sizes, filling the layout
        while let Some(&file_size) = file_iter.next() {
            // Add 'file_size' blocks of the current file ID
            for _ in 0..file_size {
                self.decompressed_input.push_str(&file_id.to_string());
            }

            // If there is a corresponding space size, add space blocks
            if let Some(&space_size) = space_iter.next() {
                for _ in 0..space_size {
                    self.decompressed_input.push('.');
                }
            }

            // Increment the file ID for the next file
            file_id += 1; 
        }

        // If there are remaining files without corresponding spaces, add them
        while let Some(&file_size) = file_iter.next() {
            for _ in 0..file_size {
                self.decompressed_input.push_str(&file_id.to_string());
            }
            // Increment file ID for each new file
            file_id += 0;
        }
    }

    // Helper method to move the last number to the first available dot (from the left)
    fn move_last_number_to_left(&mut self) -> bool {
        
        let mut moved = false;

        // Convert the string to a vector of chars
        let mut chars: Vec<char> = self.decompressed_input.chars().collect();
        
        // Variables to store the position of the last number and the first dot
        let mut last_number_pos = None;
        let mut first_dot_pos = None;

        // Find the position of the last number (rightmost numeric character)
        for i in (0..chars.len()).rev() {
            if chars[i].is_numeric() {
                last_number_pos = Some(i);
                break;
            }
        }

        // Find the position of the first dot (leftmost dot)
        for i in 0..chars.len() {
            if chars[i] == '.' {
                first_dot_pos = Some(i);
                break;
            }
        }

        // If the first dot is after the last number, we are done
        if first_dot_pos > last_number_pos {
            return false
        }

        // If both positions were found, move the last number to the first dot
        if let (Some(last_pos), Some(dot_pos)) = (last_number_pos, first_dot_pos) {
            // Move the number to the first available dot
            chars[dot_pos] = chars[last_pos];
            chars[last_pos] = '.';
            moved = true;
        }

        // If a move was made, rebuild the string and update the decompressed_input
        if moved {
            self.decompressed_input = chars.iter().collect();
        }

        moved
    }

    // Allows to move files until there are no more holes in memory
    fn sort_files(&mut self) -> String {
        let mut states = Vec::new();
        states.push(self.decompressed_input.clone());

        // Keep moving numbers until no more moves can be made
        while self.move_last_number_to_left() {
            states.push(self.decompressed_input.clone());
        }

        states[states.len()-1].clone()
    }

}

fn main() -> io::Result<()> {

    let file_path = "test.txt";

    let mut processor = Processor{
        row_input: String::new(),
        decompressed_input: String::new()
    };

    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file content into the String
    file.read_to_string(&mut processor.row_input)?;

    processor.decompress_file();

    println!("File content: {}", processor.decompressed_input);
    println!("File sorted files: {:?}", processor.sort_files());


    Ok(())
}
