mod randomness_tests;
mod tests_for_tests_lol;

use rand::Rng;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    let mut data_to_check: Vec<u8> = Vec::new();

    // let mut file = File::create("poker_test.txt").unwrap();

    // it doesn't make sense to store the data in u64, since 20000 bits = 312.5 64-bit integers...
    // no need to introduce corner cases with paddings unless performance is absolutely crucial
    for i in 0..2500 {
        let value: u8 = rng.gen_range(0..(u16::pow(2, 8) - 1)) as u8;
        data_to_check.push(value);
    }
    // file.write(data_to_check.as_slice()).unwrap();



    let test1: bool = randomness_tests::monobit_test(&data_to_check);
    let test2: bool = randomness_tests::max_series_length_test(&data_to_check);
    let test3: bool = randomness_tests::poker_test(&data_to_check);
    let test4: bool = randomness_tests::series_length_test(&data_to_check);
    println!("{} {} {} {}", test1, test2, test3, test4);
}
