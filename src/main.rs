use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() {
    let filename = "resources/DIE-DOK_IDs_alle.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<_> = reader.lines().collect();
    let line_number = lines.len();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in lines.into_iter().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        print!("Fid={}", line);
        
        if index + 1 < line_number {
            print!("|");
        }
        
    }

    println!("");

    print_to_stdout(&mut reader);
}



fn print_to_stdout(mut input: &mut Read) {
    let mut stdout = io::stdout();
    io::copy(&mut input, &mut stdout);
}
