extern crate chrono;
use chrono::Timelike;

use chrono::offset::Utc;
use chrono::DateTime;
use std::fs;
use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn main() {
    let mut histo: BTreeMap<DateTime<Utc>, i32> = BTreeMap::new();

    let mut count = 0;
    for line in io::stdin().lock().lines() {
        let filename = line.unwrap();
        consume(&filename, &mut histo);
        count+=1;
    }

    // Iterate over histo
    for(key, value) in histo.iter() {
        println!("{},{}", key.format("%F %T"), value);
    }
    eprintln!("consumed {} files, into {} quanta", count, histo.keys().len());

}

fn consume(filename: &str, histo: &mut BTreeMap<DateTime<Utc>, i32>) {
    let mt = fs::metadata(filename).unwrap();
    let datetime: DateTime<Utc> = mt.created().unwrap().into();

    let map_key = datetime
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap();

    let count = histo.entry(map_key).or_insert(0);
    *count += 1;
}
