use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

extern crate curl;
extern crate serde_json;
extern crate clap;

use curl::http;
use serde_json::Value;
use clap::{Arg,App};


fn main() {

    let matches = App::new("Matomo export filter")
    .version("0.1.0")
    .author("Hermann Schwarz")
    .about("Parses matomo export and filters it to get only special statistics")
    .arg(Arg::with_name("url")
        .short("u")
        .long("url")
        .takes_value(true)
        .help("url of the export"))
    .get_matches();

    let url = matches.value_of("url").unwrap();
    println!("input URL: {}", url);


//  read_ids();
  read_json(url);
}


fn read_ids() {
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
}


/*
* try to read JSON with curl::http
* https://docs.rs/curl/0.5.0/curl/
* https://hermanradtke.com/2015/09/21/get-data-from-a-url-rust.html
*/
fn read_json(url : &str) {

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

    let json: Value = serde_json::from_str(body).unwrap_or_else(|e| {
        panic!("Failed to parse json; error is {}", e);
    });

    let json_len = json.as_array().unwrap().len();
    println!("JSON len: {}", json_len);

    for i in 0..json_len {
        let label = &json[i]["label"];
        println!("Label: {}", label);

        if label == "literatur" { // parse only in DOMAIN/literatur/*
            let subtable = &json[i]["subtable"];
            let subtable_size = subtable.as_array().unwrap().len();
            println!("subtable size: {}", subtable_size); 
            break;
        }
    }

}

