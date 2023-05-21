use std::cmp::max;
use std::collections::HashMap;

fn fetch_bit_from_position(byte: u8, bit_index: u8) -> u8 {
    assert!(0 <= bit_index && bit_index <= 7);
    (byte & (1 << bit_index)) >> bit_index
}

pub fn monobit_test(data: &Vec<u8>) -> bool {
    let mut zero_counter = 0;
    let mut one_counter = 0;
    for byte in data {
        for bit_index in 0..8 {
            if fetch_bit_from_position(*byte, bit_index) == 1 {
                one_counter += 1;
            } else {
                zero_counter += 1;
            }
        }
    }
    let left_conf_interval_boundary = 9654;
    let right_conf_interval_boundary = 10346;
    let does_pass_test =
        left_conf_interval_boundary < zero_counter && zero_counter < right_conf_interval_boundary
            &&
        left_conf_interval_boundary < one_counter && one_counter < right_conf_interval_boundary;

    does_pass_test
}

pub fn max_series_length_test(data: &Vec<u8>) -> bool {
    assert!(data.len() > 0);

    let mut max_length: u32 = 0;
    let mut current_length: u32 = 0;
    let mut previous_bit: i8 = -1;
    let mut i = 0;
    while i < data.len() {
        for bit_index in 0..8 {
            let bit: u8 = fetch_bit_from_position(data[i], bit_index);
            if previous_bit == -1 {
                previous_bit = bit as i8;
                current_length = 1;
            } else if previous_bit == (bit as i8) {
                current_length += 1;
            } else {
                previous_bit ^= 1; // invert the bit
                current_length = 1;
            }
        }
        max_length = max(max_length, current_length);
        // println!("{} {}", current_length, max_length);
        i += 1;
    }
    let does_pass_test: bool = max_length <= 36;

    does_pass_test
}

pub fn poker_test(data: &Vec<u8>) -> bool {
    let mut sequence_counter: HashMap<u8, u32> = HashMap::new();
    let m = 4;
    let k = data.len() as u32 * 2; // since m = 4, the number of 4-bit chunks is equal to (number of 8-bit chunks) * 2
    for byte in data {
        for bit_shift_for_tetrad in (0..8).step_by(4) {
            let tetrad: u8 = (byte >> bit_shift_for_tetrad) & 0x0f; // will be in the form of 0000xyzw in bit representation
            let count = sequence_counter.entry(tetrad).or_insert(0);
            *count += 1;
        }
    }
    let square_sum = sequence_counter.iter().fold(0, |acc, x| {
        println!("{}", x.0);
        acc + x.1 * x.1
    });
    let chi_squared_statistic: f64 = ((u32::pow(2, m) * square_sum) as f64 / k as f64) - k as f64;
    let left_conf_interval_boundary = 1.03;
    let right_conf_interval_boundary = 57.4;

    println!("{chi_squared_statistic}");
    let does_pass_test =
        left_conf_interval_boundary < chi_squared_statistic &&
        chi_squared_statistic < right_conf_interval_boundary;

    does_pass_test
}

pub fn series_length_test(data: &Vec<u8>) -> bool {
    series_checker_for_specific_bit(data, 0) && series_checker_for_specific_bit(data, 1)
}

pub fn series_checker_for_specific_bit (data: &Vec<u8>, bit_to_check: i8) -> bool {
    let mut sequence_length_counter: HashMap<u32, u32> = HashMap::new();
    let benchmark: HashMap<u32, (u32, u32)> = HashMap::from(
        [
            (1, (2267, 2733)),
            (2, (1079, 1421)),
            (3, (502, 748)),
            (4, (223, 402)),
            (5, (90, 223)),
            (6, (90, 223))
        ]
    );
    let mut previous_bit = -1;
    let mut current_length = 0;
    for byte in data {
        for bit_index in 0..8 {
            let bit = fetch_bit_from_position(*byte, bit_index);
            if previous_bit == -1 {
                previous_bit = bit as i8;
                current_length = 1;
            } else if previous_bit == (bit as i8) {
                current_length += 1;
            } else {
                if previous_bit == bit_to_check {
                    *sequence_length_counter.entry(current_length).or_insert(0) += 1;
                }
                previous_bit ^= 1; // invert the bit
                current_length = 1;
            }
        }
    }
    let mut does_pass_test  = true;
    println!();
    let mut distribution_tail_counter = 0;
    for (key, val) in sequence_length_counter {
        println!("{key} {val}");
        let mut current_key = key;
        if current_key >= 6 {
            distribution_tail_counter += val;
        } else {
            let confidence_interval = benchmark.get(&current_key).unwrap();
            if !(confidence_interval.0 <= val && val <= confidence_interval.1) {
                does_pass_test = false;
            }
        }
    }
    println!();
    let confidence_interval = benchmark.get(&6).unwrap();
    if !(confidence_interval.0 <= distribution_tail_counter  && distribution_tail_counter <= confidence_interval.1) {
        does_pass_test = false;
    }

    does_pass_test
}