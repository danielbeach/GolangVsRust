use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() {
    let _ = count_words();
}

fn count_words() -> Result<HashMap<String, i32>, String> {
    let rdr = BufReader::new(File::open("confessions.txt").expect("unable to open file!"));
    let mut word_counts = HashMap::new();
    let _: Vec<_> = rdr.lines().map(|mut line| {
            line = Ok(line.unwrap().replace(".", "").replace(",", "").replace(";", "").replace("?", "").replace("!", "").replace("a", "").replace("the", "")); // could be a regex replace here, but using pure rust so hey ho
            line.unwrap().split_whitespace().for_each(|word| { *(word_counts.entry(word.to_string()).or_insert(0)) += 1; });
        }).collect();
    let _ = &word_counts.iter().filter(|&(_,v)| v > &100).for_each(|(k,v)| println!("{} / {}", k, v) );
    Ok(word_counts)
}