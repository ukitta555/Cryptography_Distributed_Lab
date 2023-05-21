#[cfg(test)]
mod tests {
    use std::fs;
    use crate::randomness_tests;

    #[test]
    fn test_monobit_test() {
        let mut test_data: Vec<u8> = Vec::new();
        for i in 0..1250 {
            test_data.push(0);
            test_data.push(255);
        }

        assert!(randomness_tests::monobit_test(&test_data));
    }

    #[test]
    fn test_max_series_length_test() {
        let mut test_data: Vec<u8> = Vec::new();
        let mut index: u32;
        for index  in 0..2495 {
            // println!("{} {}", index % 255, (index % 255) as u8);
            test_data.push((index % 255) as u8);
        }
        test_data.push(255);
        test_data.push(255);
        test_data.push(255);
        test_data.push(255);
        test_data.push(255);
        assert!(!randomness_tests::max_series_length_test(&test_data));

        let mut test_data: Vec<u8> = Vec::new();
        let mut index: u32;
        for index  in 0..2495 {
            // println!("{} {}", index % 255, (index % 255) as u8);
            test_data.push((index % 255) as u8);
        }
        test_data.push(0);
        test_data.push(1);
        test_data.push(2);
        test_data.push(3);
        test_data.push(4);
        assert!(randomness_tests::max_series_length_test(&test_data));
    }

    #[test]
    fn test_poker_test() {
        let mut test_data: Vec<u8> = Vec::new();
        let mut index: u32;
        let test_data = fs::read("poker_test.txt").unwrap().to_vec();

        assert!(randomness_tests::poker_test(&test_data));
    }



    #[test]
    fn test_series_length_test() {
        let mut test_data: Vec<u8> = Vec::new();
        let mut index: u32;
        for index  in 0..2495 {
            test_data.push((index % 255) as u8);
        }
        test_data.push(0);
        test_data.push(1);
        test_data.push(2);
        test_data.push(3);
        test_data.push(4);
        assert!(randomness_tests::max_series_length_test(&test_data));
    }
}
