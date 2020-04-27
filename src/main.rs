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
    .arg(Arg::with_name("dir1")
        .short("d1")
        .long("dir1")
        .takes_value(true)
        .help("Directory name in www.DOMAIN/<DIR>"))
    .arg(Arg::with_name("filter")
        .short("f")
        .long("filter")
        .takes_value(true)
        .help("Filter string in DIR2 www.DOMAIN/<DIR>/<DIR2>.  E.g. in 'www.example.com/user/my_user_profile' and 'www.example.com/user/my' you can filter on 'profile' to get first one."))
    .get_matches();

    let url = matches.value_of("url").unwrap_or("URL is not set");
    let dir1 = matches.value_of("dir1").unwrap_or("dir1 is not set");
    let filter = matches.value_of("filter").unwrap_or("filter is not set");

    println!("input URL: {}", url);


/*
  let ids = read_ids();
  for (index, id) in ids.into_iter().enumerate() {
      let line = line.unwrap();
      print!("FId={}", line);
  }
*/

  read_json(url, dir1, filter);
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
fn read_json(url : &str, dir1 : &str, filter : &str) {

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
        println!("DIR1: DOMAIN/{}", label);

       // if label == "literatur" { // parse only in DOMAIN/literatur/*
        if label == dir1 { // parse only in DOMAIN/bookmark/* (if dir1 = 'bookmark')

            let bookmarks_nb_hits = &json[i]["nb_hits"];
            println!("Hits in DOMAIN/{}: {}", dir1, bookmarks_nb_hits); 

            let subtable_array = &json[i]["subtable"];
            let subtable_size = subtable_array.as_array().unwrap().len();
            println!("Amount of sites (subtable size): {}", subtable_size); 

            let mut hits_sum: i32 = 0;

            for j in 0..subtable_size {
                let subtable_label = &subtable_array[j]["label"];
                let subtable_label_string = subtable_label.to_string();

                let subtable_url = &subtable_array[j]["url"];
                println!("URL: {}", subtable_url);

                if subtable_label_string.contains(filter) { // parse only in DOMAIN/bookmark/32290/ (if e.g. filter = 32290)
                    println!("Script found: {} filtered by {}", subtable_label, filter);

                    let hits_string = &subtable_array[j]["nb_hits"].to_string();
                    let hits_integer = hits_string.parse::<i32>().unwrap();
                    println!("HITS: {}", hits_integer);
                    hits_sum = hits_sum + hits_integer;
                //    break; // don't break because we want to calculate hits sum for all the found (by filtering) scripts
                }
            }

            println!("Sum of HITS: {}", hits_sum);

            break; // use break because we're finished (reached "/bookmarks")
        }
    }

}

