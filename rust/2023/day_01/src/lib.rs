pub mod input;
pub mod part_1;
pub mod part_2;

// add the numbers in a vector of u32
//
// this function is general for the day one, that's why it's staying here
pub fn sum_calibration_numbers(numbers: Vec<u32>) -> u32 {
    numbers.iter().sum()
}
