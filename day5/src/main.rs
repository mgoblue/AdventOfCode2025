#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::fs;
use regex::Regex;

#[derive(Copy, Clone)]
struct Range {
    start: i64,
    end: i64,
}

// part 1 - 828
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let parts: Vec<&str> = contents.split("\r\n\r\n").collect();

    let ranges : Vec<Range> = parseRanges( parts[0] );

    let mut countFreshIds = 0;
    for id in parts[1].lines() {
        let idNum: i64 = id.parse().unwrap();
        for range in &ranges { // I don't understand why I need to borrow here
            if idNum >= range.start && idNum <= range.end {
                countFreshIds += 1;
                break;
            }
        }
    }
    println!("countFreshIds: {}", countFreshIds);
}

fn parseRanges( rangeParts: &str ) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();

    for line in rangeParts.lines() {
        //println!("line: {}", line);
        if let Some(caps) = re.captures(line) {
            let start: i64 = caps[1].parse().unwrap();
            let end: i64 = caps[2].parse().unwrap();
            ranges.push(Range { start, end });
        }
    }

    return ranges;
}