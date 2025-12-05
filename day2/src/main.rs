use std::fs;

fn main() {
    //part 1 - 40055209690
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let parts: Vec<&str> = contents.split(',').collect();
    let mut sum_invalid_id: i128 = 0;
    for part in parts {
        //println!("Part: {}", part);
        let range_start_str = part.split('-').next().unwrap();
        let range_end_str = part.split('-').nth(1).unwrap();
        //println!("Range Start: {}, Range End: {}", range_start_str, range_end_str);

        let range_start: i64 = range_start_str.parse().unwrap();
        let range_end: i64 = range_end_str.parse().unwrap();
        //println!("Parsed Range Start: {}, Parsed Range End: {}", range_start, range_end);

        for number in range_start..=range_end {
            let number_str = number.to_string();
            let length = number_str.len();
            if length % 2 != 0 {
               continue;
            }

            let half_length = length / 2;
            let first_half_str = &number_str[..half_length];
            let second_half_str = &number_str[half_length..];

            if first_half_str == second_half_str {
                //println!("Invalid number (both halves equal): {}", number);
                sum_invalid_id += number as i128;
                //println!( "Current Sum of Invalid IDs: {}", sum_invalid_id);
            }
        }
    }
    println!("Final Sum of invalid IDs: {}", sum_invalid_id);
}
