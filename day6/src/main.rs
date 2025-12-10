#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use std::fs;

// part 1 - 5524274308182
// part 2 - 8843673199391
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut worksheet = Vec::<Vec::<char>>::new();

    for line in contents.lines() {
        let chars: Vec<char> = line.chars().collect();
        worksheet.push(chars);
    }

    let mut operator = worksheet.last().unwrap()[0];
     for i in 0..worksheet[0].len(){
        if worksheet.last().unwrap()[i] != ' ' {
            operator = worksheet.last().unwrap()[i];
        } else {
           worksheet.last_mut().unwrap()[i] = operator;
        }
    }
    //println!( "operators {:?}", worksheet.last().unwrap() );
    //println!( "worksheet {:?}", worksheet);

    let mut sum = 0;
    let mut operator = worksheet.last().unwrap()[0];
    let mut result: i128 = 0;
    if operator == '*'{
       result = 1;
    }
    for i in 0..worksheet[0].len() {
        let col = worksheet[..worksheet.len()-1].iter().map(|row| row[i]).collect::<Vec<char>>();
        if !hasNumber(&col) {
            println!("result {}", result);
            sum += result;
            operator = worksheet.last().unwrap()[i+1];
            result = 0;
            if operator == '*'{
                result = 1;
            }

            continue;
        }

        let number = findNumber(&col);
        //println!("Column {:?} operator {}", col, operator);

        match operator {
            '+' => result += number,
            '*' => result *= number,
            _ => (),
        }
        //println!("number {} result {}", number, result);
    }
    sum += result;
    println!("final result {}", sum);
    // let mut sum = 0;

    //     sum += result;
    // }
    // println!("sum {}", sum);
}

fn findNumber(col: &Vec<char>) -> i128 {
    let mut number_str = String::new();
    for &c in col {
        if c.is_digit(10) {
            number_str.push(c);
        }
    }
    let number: i128 = number_str.parse().unwrap();
    println!("Found number {}", number);
    return number;
}

fn hasNumber(col: &Vec<char>) -> bool {
    for &c in col {
        if c.is_digit(10) {
            return true;
        }
    }
    return false;
}
