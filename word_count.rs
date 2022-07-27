use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let rdr = BufReader::new(File::open("confessions.txt").expect("problem"));
    let to_replace = vec![".", ",", ";", "?", "!", "a", "the"];
    let mut word_counts = HashMap::new();
    for line in rdr.lines() {
        if let Ok(mut ln) = line {
            for rpl in to_replace.iter() {
                ln = ln.replace(rpl, "");
                }
            for word in ln.split_whitespace() {
                    *(word_counts.entry(word.to_string()).or_insert(0)) += 1;
            }
        }
    }
    for (k, v) in word_counts {
        if v > 100 {
            println!("{} / {}", k, v);
        }
    }
    Ok(())
}
