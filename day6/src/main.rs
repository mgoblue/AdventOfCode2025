#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::fs;

// part 1 - 5524274308182
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut numbers = Vec::<Vec::<i64>>::new();

    for i in 0..contents.lines().count()-1 {
        numbers.push(Vec::<i64>::new());
        let line = contents.lines().nth(i).unwrap();
        let mut numberStrings : Vec::<&str> = line.split_whitespace().collect();
        for numberString  in &numberStrings {
            let num = numberString.parse::<i64>().unwrap();
            numbers[i].push(num);
        }
    }

    let operators: Vec<char> = contents.lines().last().unwrap()
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect();

    let mut sum = 0;
    for i in 0..operators.len() {
        let mut result = numbers[0][i];
        for j in 1..numbers.len() {
            let operator = operators[i];
            let nextNumber = numbers[j][i];
            match operator {
                '+' => result += nextNumber,
                '-' => result -= nextNumber,
                '*' => result *= nextNumber,
                '/' => result /= nextNumber,
                _ => (),
            }
        }
        sum += result;
    }
    println!("sum {}", sum);
}
