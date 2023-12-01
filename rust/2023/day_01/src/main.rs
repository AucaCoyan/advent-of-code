/// Day 1 Treuchet?!
/// https://adventofcode.com/2023/day/1
///
mod input;

#[allow(unused_imports)]
use input::INPUT;

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

fn main() {
    calibrate_string("1abc2");
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn first_sample() {
        assert_eq!(calibrate_string("1abc2"), 12);
    }
    #[test]
    fn second_sample() {
        assert_eq!(calibrate_string("pqr3stu8vx"), 38);
    }
    #[test]
    fn third_sample() {
        assert_eq!(calibrate_string("a1b2c3d4e5f"), 15);
    }
    #[test]
    fn fourth_sample() {
        assert_eq!(calibrate_string("treb7chet"), 77);
    }
}
