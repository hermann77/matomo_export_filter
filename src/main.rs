// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Read};

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
    .about("Parses matomo JSON export file and filters it to get only special statistics")
    .arg(Arg::with_name("url")
        .short("u")
        .long("url")
        .takes_value(true)
        .help("url of the export"))
    .get_matches();

    let url = matches.value_of("url").unwrap_or("URL is not set");
    println!("input URL: {}", url);


/*
  let ids = read_ids();
  for (index, id) in ids.into_iter().enumerate() {
      let line = line.unwrap();
      print!("FId={}", line);
  }
*/

  read_json(url);
}

/**
 *  Reading a text file with IDs
 */
/*
fn read_ids() -> std::vec::Vec<std::result::Result<std::string::String, std::io::Error>> {
    let filename = "resources/dok_ids.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let ids: Vec<_> = reader.lines().collect();
    return ids;
}
*/

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

       // if label == "literatur" { // parse only in DOMAIN/literatur/*
        if label == "bookmark" { // parse only in DOMAIN/bookmark/*

            let bookmarks_nb_hits = &json[i]["nb_hits"];
            println!("Hits in bookmarks: {}", bookmarks_nb_hits); 

            let subtable_array = &json[i]["subtable"];
            let subtable_size = subtable_array.as_array().unwrap().len();
            println!("subtable size: {}", subtable_size); 

            for j in 0..subtable_size {
                let subtable_label = &subtable_array[j]["label"];
                let subtable_label_string = subtable_label.to_string();

                if subtable_label_string.contains("32290") { // parse only in DOMAIN/bookmark/32290/*
                    println!("subtable label: {}", subtable_label);

                    let hits = &subtable_array[j]["nb_hits"];
                    println!("HITS in 32290: {}", hits);

                    break;
                }
            }

            break; // use break because we're finished (reached "/bookmarks")
        }
    }

}

