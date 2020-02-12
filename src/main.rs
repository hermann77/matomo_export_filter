use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};


extern crate curl;
extern crate serde_json;

use serde_json::Value;
use curl::easy;


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

  //  print_to_stdout(&mut reader);


  read_json();
}

/*
* try to read JSON with curl::http
* https://docs.rs/curl/0.5.0/curl/
* https://hermanradtke.com/2015/09/21/get-data-from-a-url-rust.html
*/
fn read_json() {

    let url = "https://www.hautelook.com/api";
    let resp = http::handle()
        .get(url)
        .exec()
        .unwrap_or_else(|e| {
            panic!("Failed to get {}; error is {}", url, e);
        });

    if resp.get_code() != 200 {
        println!("Unable to handle HTTP response code {}", resp.get_code());
        return;
    }

    let body = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
        panic!("Failed to parse response from {}; error is {}", url, e);
    });

}



/*
fn print_to_stdout(mut input: &mut Read) {
    let mut stdout = io::stdout();
    io::copy(&mut input, &mut stdout);
}
*/
