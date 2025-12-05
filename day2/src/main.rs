use std::fs;

fn is_invalid(number: i64, output: &mut String) -> bool {
    let number_str = number.to_string();
    let length = number_str.len();

    if length == 1 {
        return false;
    }

     let half_length = length.div_ceil(2);
    for i in 0..half_length {
        let pattern = &number_str[..i+1];
        //output.push_str(&format!("Pattern: {}\n", pattern));

        let repeated = pattern.repeat(length / pattern.len());
        if repeated == number_str {
            output.push_str(&format!("Invalid number (repeated pattern): {} and pattern {}\n", number, pattern));
            return true;
        }
    }

    return false;
}

fn main() {
    //part 1 - 40055209690
    //part 2 - 50857215650
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut output = String::new();

    let parts: Vec<&str> = contents.split(',').collect();
    let mut sum_invalid_id: i128 = 0;
    for part in parts {
        //println!("Part: {}", part);
        let range_start_str = part.split('-').next().unwrap();
        let range_end_str = part.split('-').nth(1).unwrap();
        //println!("Range Start: {}, Range End: {}", range_start_str, range_end_str);

        let range_start: i64 = range_start_str.parse().unwrap();
        let range_end: i64 = range_end_str.parse().unwrap();
        output.push_str(&format!("Range Start: {}, Range End: {}\n", range_start, range_end));

        let mut count_of_invalid_in_range: i32 = 0;
        for number in range_start..=range_end {
            if is_invalid(number, &mut output){
                count_of_invalid_in_range += 1;
                
                //println!("Invalid number: {}", number);
                sum_invalid_id += number as i128;
                //println!( "Current Sum of Invalid IDs: {}", sum_invalid_id);
            }
        }
        println!("Count of invalid numbers in range {}-{}: {}", range_start, range_end, count_of_invalid_in_range);
    }
    println!("Final Sum of invalid IDs: {}", sum_invalid_id);
    output.push_str(&format!("Final Sum of invalid IDs: {}\n", sum_invalid_id));
    fs::write("output.txt", &output).expect("Failed to write to output.txt");
}
