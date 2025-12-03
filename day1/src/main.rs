#![allow(unused_variables)]

use std::fs;

fn main() {
    //part 1 - 1059
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut currentPosition = 50;
    let mut newPosition = 0;
    let mut count = 0;

    for line in contents.lines() {
        println!("current position: {}", currentPosition);
        println!("{}", line);

        let line = line.trim();
        if !line.is_empty() {
            let direction = line.chars().next().unwrap();
            let distance: i32 = line[1..].parse().unwrap();

            println!("Direction: {}, Distance: {}", direction, distance);

            let mut temp: i32 = 0;
            if direction == 'L' {
                print!("Moving Backward to position: ");
                temp = ( currentPosition - distance );
            } else if direction == 'R' {
                print!("Moving Forward to position: ");
                temp = ( currentPosition + distance );
            }

            newPosition = (temp) % 100;
            if newPosition < 0 {
                newPosition += 100;
            }

            println!("new position {}", newPosition);
            println!("temp {}", temp );

            if newPosition == 0 {
                count += 1;
            }

            currentPosition = newPosition;
        }
    }

    println!("Count of position at 0: {}", count);
}
