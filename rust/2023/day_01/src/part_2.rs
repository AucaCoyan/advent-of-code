use regex::Regex;

use crate::sum_calibration_numbers;

#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(dead_code)]

const NUMBERS: &[&str; 9] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_number(number_as_str: &str) -> u32 {
    if number_as_str.chars().all(|b| b.is_ascii_digit()) {
        number_as_str.parse::<u32>().unwrap()
    } else {
        match number_as_str {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        }
    }
}

pub fn calibrate_string(sentence: &str) -> u32 {
    let pattern = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    let results: Vec<&str> = pattern.find_iter(sentence).map(|m| m.as_str()).collect();
    // dbg!(&results);
    // println!("{:?}", &results.first());
    let initial = parse_number(results.first().unwrap());
    let _final = parse_number(results.last().unwrap());
    10 * initial + _final
}

// given a str with \n in it, split the lines, get the vales and add them all
pub fn split_and_sum_p2(input: &str) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    let lines = input.lines().collect::<Vec<&str>>();
    for line in lines {
        numbers.push(calibrate_string(line));
    }
    sum_calibration_numbers(numbers)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_line() {
        assert_eq!(calibrate_string("eighthree"), 83);
    }

    #[test]
    fn test_single_lines() {
        assert_eq!(calibrate_string("two1nine"), 29);
        assert_eq!(calibrate_string("eightwothree"), 83);
        assert_eq!(calibrate_string("abcone2threexyz"), 13);
        assert_eq!(calibrate_string("xtwone3four"), 24);
        assert_eq!(calibrate_string("4nineeightseven2"), 42);
        assert_eq!(calibrate_string("zoneight234"), 14);
        assert_eq!(calibrate_string("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_sum_of_lines() {
        assert_eq!(
            sum_calibration_numbers(vec![29, 83, 13, 24, 42, 14, 76]),
            281
        );
    }

    #[test]
    fn test_steps_1_and_2() {
        assert_eq!(
            split_and_sum_p2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}
