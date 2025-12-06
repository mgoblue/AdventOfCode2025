#![allow(unused_variables)]
use std::fs;

// part 1 - 17359
// part 2 - 172787336861064
fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut sum_jolatage: i64 = 0;

    for number_str in contents.lines() {
        sum_jolatage += find_joltage( number_str );
    }

    println!("Sum of joltages: {}", sum_jolatage);
}

fn find_joltage( number_str : &str ) -> i64{
    //println!("Line: {}", number_str);

    let mut joltage: i64 = 0;
    let number_of_digits = 12;

    let str_len = number_str.len() as i32;
    let mut start_index = 0;
    //println!("String length: {}", str_len);
    for i in (0..number_of_digits).rev() {
        let up_to_index = str_len - i;
        //println!("Iteration: {}, up_to_len: {}", i, up_to_len);
        let largest_digit = find_largest_digit( number_str, &mut start_index, up_to_index );
        //println!("Largest digit in iteration {}: {}", i, largest_digit);

        joltage = joltage * 10 + largest_digit as i64;
    }
    //println!("=> Current joltage: {}", joltage);
    return joltage;
}

fn find_largest_digit( number_str: &str, start_index: &mut i32, up_to_index: i32 ) -> i32 {
    //println!("Finding largest digit from index {} to {}", *start_index, up_to_index);
    let mut largest: i32 = 0;
    for i in *start_index..up_to_index {
        let num: i32 = number_str.chars().nth(i as usize).unwrap().to_digit(10).unwrap() as i32;
        if num > largest {
             largest = num;
             *start_index = i + 1;
        }
    }
    //println!("=> Largest digit found: {} at index {}", largest, start_index);
    return largest;
}