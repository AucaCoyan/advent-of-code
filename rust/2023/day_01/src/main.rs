/// Day 1 Treuchet?!
/// https://adventofcode.com/2023/day/1
///
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(dead_code)]
mod input;

use input::INPUT;

// takes a `sentence: &str` and returns the calibration number
fn calibrate_string(sentence: &str) -> u32 {
    let mut initial = 0;
    let mut _final = 0;
    for char in sentence.chars() {
        if char.is_numeric() {
            let number = char.to_digit(10).unwrap();
            // dbg!(&number);
            if initial == 0 {
                initial = number
            } else {
                _final = number
            }
        }
    }
    if _final == 0 && initial != 0 {
        _final = initial
    }
    10 * initial + _final
}

// add the numbers in a vector of u32
fn sum_calibration_numbers(numbers: Vec<u32>) -> u32 {
    numbers.iter().sum()
}

// given a str with \n in it, split the lines, get the vales and add them all
fn split_and_sum(input: &str) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    let lines = input.lines().collect::<Vec<&str>>();

    for line in lines {
        numbers.push(calibrate_string(line));
    }
    sum_calibration_numbers(numbers)
}

fn main() {
    println!("{}", split_and_sum(INPUT));
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::*;

    #[test]
    fn test_single_lines() {
        assert_eq!(calibrate_string("1abc2"), 12);
        assert_eq!(calibrate_string("pqr3stu8vx"), 38);
        assert_eq!(calibrate_string("a1b2c3d4e5f"), 15);
        assert_eq!(calibrate_string("treb7chet"), 77);
    }
    #[test]
    fn test_sum_of_lines() {
        assert_eq!(sum_calibration_numbers(vec![12, 38, 15, 77]), 142);
    }

    #[test]
    fn test_steps_1_and_2() {
        assert_eq!(calibrate_string("pqr3stu8vx"), 38);
    }
}
