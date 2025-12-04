#![allow(unused_variables)]

use std::fs;

fn main() {
    //part 1 - 1059
    //part 2 - 6305
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut output = String::new();

    let mut currentPosition = 50;
    let mut newPosition = 0;
    let mut count = 0;
    let mut countAtZero = 0;

    for line in contents.lines() {
        output.push_str(&format!("current position: {}\n", currentPosition));
        output.push_str(&format!("{}\n", line));

        let line = line.trim();
        if !line.is_empty() {
            let direction = line.chars().next().unwrap();
            let mut distance: i32 = line[1..].parse().unwrap();
            //output.push_str(&format!("Direction: {}, Distance: {}\n", direction, distance));

            let rotation = distance / 100;
            output.push_str(&format!("rotations: {}\n", rotation ));

            distance = distance % 100;
            //output.push_str(&format!("Adjusted Distance: {}\n", distance ));
            //output.push_str(&format!("Direction: {}, Distance: {}\n", direction, distance));
            let mut temp: i32 = 0;
            if direction == 'L' {
                count += rotation.abs();
                if currentPosition != 0 && ( currentPosition - distance ) <= 0 {
                    count += 1;
                    output.push_str(&format!("Passed position 0! count is {}\n", count));
                }
                temp = ( currentPosition - distance );
                //output.push_str(&format!("Moving Backward to position: \n"));
            } else if direction == 'R' {
                count += rotation.abs();
                if currentPosition != 0 && ( currentPosition + distance ) >= 100 {
                    count += 1;
                    output.push_str(&format!("Passed position 0! count is {}\n", count));
                }
                temp = ( currentPosition + distance );
                //output.push_str(&format!("Moving Forward to position: \n"));
            }

            newPosition = (temp) % 100;
            //output.push_str(&format!("new position {}\n", newPosition));
            if newPosition < 0 {
                newPosition += 100;
                //output.push_str(&format!("new adjusted position {}\n", newPosition));
            }
            output.push_str(&format!("new position {}\n", newPosition));
            if newPosition == 0 {
                countAtZero += 1;
                // output.push_str(&format!("At position 0! countAtZero is {}\n", countAtZero));
            }

            currentPosition = newPosition;
        }
    }

    output.push_str(&format!("At position 0! countAtZero is {}\n", countAtZero));
    output.push_str(&format!("Count of position passing 0: {}\n", count));
    fs::write("output.txt", &output).expect("Failed to write to output.txt");
}