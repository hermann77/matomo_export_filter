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

    let url = matches.value_of("url").unwrap_or("URL is not set");
    println!("input URL: {}", url);



  let lines = read_ids();
  for (index, line) in lines.into_iter().enumerate() {
      let line = line.unwrap();
      print!("Fid={}", line);
  }
//  read_json(url);
}


fn read_ids() -> std::vec::Vec<std::result::Result<std::string::String, std::io::Error>> {
    let filename = "resources/dok_ids.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();
    return lines;
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

