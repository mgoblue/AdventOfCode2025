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
// part 2 - 352681648086146
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let parts: Vec<&str> = contents.split("\r\n\r\n").collect();

    let ranges : Vec<Range> = parseRanges( parts[0] );
    let sortedRanges = sortRanges( &ranges );
    let mergedRanges = mergeRanges( &sortedRanges );

    let mut countFreshIds = 0;
    for range in &mergedRanges {
        countFreshIds += range.end - range.start + 1;
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

fn sortRanges( ranges: &Vec<Range> ) -> Vec<Range> {
    let mut sortedRanges = ranges.clone();
    sortedRanges.sort_by_key(|r| r.start );
    return sortedRanges;
}

fn mergeRanges( sortedRanges: &Vec<Range> ) -> Vec<Range> {
    let mut mergedRanges: Vec<Range> = Vec::new();

    let mut lastRange = sortedRanges[0];
    let mut lastIndex = 0;
    for range in sortedRanges {
        if mergedRanges.is_empty() {
            mergedRanges.push(*range);
        } else {
            if range.start <= lastRange.end + 1 {
                mergedRanges[lastIndex].end = lastRange.end.max(range.end);
            } else {
                mergedRanges.push(*range);
            }
        }
        lastIndex = mergedRanges.len() - 1;
        lastRange = mergedRanges[lastIndex];
    }

    return mergedRanges;
}