// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Read};

extern crate clap;
extern crate curl;
extern crate serde_json;

use clap::{App, Arg};
use curl::http;
use serde_json::Value;

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
        .required(false)
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

/**
* try to read JSON with curl::http
* https://docs.rs/curl/0.5.0/curl/
* https://hermanradtke.com/2015/09/21/get-data-from-a-url-rust.html
*/
fn read_json(url: &str, dir1: &str, filter: &str) {
    let resp = http::handle().get(url).exec().unwrap_or_else(|e| {
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

    // @TODO: check also id right Matomo API used (&filter_pattern AND &idSubtable MUST be SET)
    if dir1.is_empty() {
        // if matomo export includes only statistics for only one subdir/subtable (Matomo API 'idSubtable') e.g. DOMAIN/<DIR1>/*
        // and additionally used a prefilter (Matomo API 'filter_pattern'):
        // e.g. www.MATOMO.com/index.php?date=2017-04-07
        //                      &expanded=1&filter_limit=-1
        //                      &format=JSON
        //                      &idSite=1
        //                      &method=Actions.getPageUrls
        //                      &module=API........
        //                      &filter_pattern=loc
        //                      &idSubtable=1236
        prase_in_one_subtable(json, filter);
    } else {
        // matomo export includes entire statistics (all the subdirectories/subtables): for DOMAIN/<DIR1>/*, DOMAIN/<DIR2>/*, DOMAIN/<DIRn>/* etc.
        // so we have to parse all the dubdirs to find the wanted dubtable
        parse_in_all_subtables(json, dir1, filter);
    }
}


/**
 *  If matomo export includes only statistics for only one subdir/subtable (Matomo API 'idSubtable')
 *    e.g. only for DOMAIN/<DIR1>/
 *    and additionally used a prefilter (Matomo API 'filter_pattern'):
 *    
 *    e.g. www.MATOMO.com/index.php?date=2017-04-07
 *                             &expanded=1&filter_limit=-1
 *                             &format=JSON
 *                             &idSite=1
 *                             &method=Actions.getPageUrls
 *                             &module=API........
 *                             &filter_pattern=loc
 *                             &idSubtable=1236
 */
fn parse_in_all_subtables(json: Value, dir1: &str, filter: &str) {
    let json_len = json.as_array().unwrap().len();
    println!("JSON len: {}", json_len);
    println!();

    for i in 0..json_len {
        let label = &json[i]["label"];

        if label == dir1 {
            // parse only in DOMAIN/<DIR1>/* (set by command line option -d)

            println!("Parsing directory DIR1: DOMAIN/{}", label);

            let bookmarks_nb_hits = &json[i]["nb_hits"];
            println!("Hits in DOMAIN/{}: {}", dir1, bookmarks_nb_hits);

            let subtable_array = &json[i]["subtable"];
            let subtable_size = subtable_array.as_array().unwrap().len();
            println!("Amount of sites (subtable size): {}", subtable_size);

            let mut hits_sum: i32 = 0;

            for j in 0..subtable_size {
                let subtable_label = &subtable_array[j]["label"];
                let subtable_label_string = subtable_label.to_string();

                if subtable_label_string.contains(filter) {
                    // parse only in DOMAIN/bookmark/32290/ (if e.g. filter = 32290)
                    let subtable_url = &subtable_array[j]["url"];
                    println!("URL: {}", subtable_url);
                    println!("Script found: {}", subtable_label);
                    println!("Filtered by {}", filter);

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


/**
 *  If Matomo export includes entire statistics (all the subdirectories/subtables): 
 *  for DOMAIN/<DIR1>/.., DOMAIN/<DIR2>/.., DOMAIN/<DIRn>/.. etc.
 *  so we have to parse all the subdirs to find the wanted dubtable
 */
fn prase_in_one_subtable(json: Value, filter: &str) {
    let json_len = json.as_array().unwrap().len();
    println!("Number of matched sites (JSON len): {}", json_len);
    println!();

    let mut hits_sum: i32 = 0;

    for i in 0..json_len {
        let label = &json[i]["label"];
        let label_string = &label.to_string();

        if label_string.contains(filter) {

            println!("Statistic for URL:{}", label);

            let nb_hits = &json[i]["nb_hits"];
            println!("Hits: {}", nb_hits);
    
            let hits_string = &nb_hits.to_string();
            let hits_integer = hits_string.parse::<i32>().unwrap();
           
            hits_sum = hits_sum + hits_integer;
        }

        println!("Sum of HITS: {}", hits_sum);
    }
}
