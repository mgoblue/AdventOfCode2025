use std::fs;

// part 1 - 17359
fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut sum_jolatage: i128 = 0;
    for line in contents.lines() {
        //println!("Line: {}", line);

        let mut joltage = 0;
        let mut largest = 0;
        let mut index = 0 as usize;

        for i in 0..line.len()-1 {
            //print!("{} ", line.chars().nth(i).unwrap());

            let num = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            if num > largest {
                largest = num;
                index = i;
            }
        }
        joltage = largest * 10;
        
        largest = 0;
        for i in index + 1 ..line.len() {
            let num = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            if num > largest {
                largest = num;
            }
        }
        joltage += largest;
        
        //println!("=> JOLTAGE: {}", joltage);
        //println!();
        sum_jolatage += joltage as i128;
    }
    println!("Sum of joltages: {}", sum_jolatage);
}
