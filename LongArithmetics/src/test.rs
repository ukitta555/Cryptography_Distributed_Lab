
#[cfg(test)]
mod tests {
    use crate::big_number::BigNumber;

    #[test]
    fn set_get_bn() {
        let mut big_num: BigNumber = BigNumber::new();
        big_num.set_hex("1023456789abcdef").unwrap();
        assert_eq!(big_num.get_hex(), "1023456789abcdef");
        big_num.set_hex("100123456789abcdef").unwrap();
        assert_eq!(big_num.get_hex(), "00000000000000100123456789abcdef");
        big_num.set_hex("101010101010100123456789abcdef").unwrap();
        assert_eq!(big_num.get_hex(), "00101010101010100123456789abcdef");
    }

    #[test]
    fn internals_bn() {
        let mut big_num: BigNumber = BigNumber::new();
        big_num.set_hex("99889987654321fedcba").unwrap(); // python check -> 39304, 11062922348666477754
        let results: [u64; 2] = [39304, 11062922348666477754];
        for (i, part) in big_num.parts.iter().enumerate() {
            assert_eq!(*part, results[i]);
        }
    }

    #[test]
    fn invert_bn() {
        let mut big_num: BigNumber = BigNumber::new();
        // invert -> 0xffffffffffff667766789abcde012345
        // f's are there because of the leading zeros in the representation.
        // element-wise sum should be equal to f for each element
        big_num.set_hex("99889987654321fedcba").unwrap();
        big_num.bit_invert();
        assert_eq!(big_num.get_hex(), "ffffffffffff667766789abcde012345");
    }

    #[test]
    fn and_bn() {
        let mut big_num_1: BigNumber = BigNumber::new();
        let mut big_num_2: BigNumber = BigNumber::new();

        big_num_1.set_hex("aaaaaaabcdefabcdef").unwrap();
        big_num_2.set_hex("aaaaaafedcbafedcba").unwrap();
        let result = big_num_1.and(&big_num_2);
        assert_eq!(result.get_hex(), "00000000000000aaaaaaaaccaaaaccaa");
    }


    #[test]
    fn or_bn() {
        let mut big_num_1: BigNumber = BigNumber::new();
        let mut big_num_2: BigNumber = BigNumber::new();

        big_num_1.set_hex("aaaaaaabcdefabcdef").unwrap();
        big_num_2.set_hex("aaaafedcbafedcba").unwrap();
        let result = big_num_1.or(&big_num_2);
        assert_eq!(result.get_hex(), "00000000000000aaaaaaffddffffddff");
    }

    #[test]
    fn xor_bn() {
        let mut big_num_1: BigNumber = BigNumber::new();
        let mut big_num_2: BigNumber = BigNumber::new();

        big_num_1.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4").unwrap();
        big_num_2.set_hex("403db8ad88a3932a0b7e8189aed9eeffb8121dfac05c3512fdb396dd73f6331c").unwrap();
        let result = big_num_1.xor(&big_num_2);
        assert_eq!(result.get_hex(), "1182d8299c0ec40ca8bf3f49362e95e4ecedaf82bfd167988972412095b13db8");
    }

    #[test]
    fn bitshift_right_bn() {
        let mut big_num_1: BigNumber = BigNumber::new();

        big_num_1.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4").unwrap();
        big_num_1.bitshift_right(132);
        assert_eq!(big_num_1.get_hex(), "051bf608414ad5726a3c1bec098f77b1");

        big_num_1.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4").unwrap();
        big_num_1.bitshift_right(129);
        assert_eq!(big_num_1.get_hex(), "28dfb0420a56ab9351e0df604c7bbd8d");

        big_num_1.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4").unwrap();
        big_num_1.bitshift_right(14);
        assert_eq!(big_num_1.get_hex(), "000146fd821052b55c9a8f06fb0263ddec6d53fec9e1fe354a29d3075ff7991c");

    }

    #[test]
    fn bitshift_left_bn() {
        let mut big_num_1: BigNumber = BigNumber::new();

        big_num_1.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4").unwrap();
        big_num_1.bitshift_left(14);
        assert_eq!(big_num_1.get_hex(), "000000000000146fd821052b55c9a8f06fb0263ddec6d53fec9e1fe354a29d3075ff7991c3a90000");

        big_num_1.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4").unwrap();
        big_num_1.bitshift_left(129);
        assert_eq!(big_num_1.get_hex(), "a37ec108295aae4d47837d8131eef636a9ff64f0ff1aa514e983affbcc8e1d4800000000000000000000000000000000");

        big_num_1.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4").unwrap();
        big_num_1.bitshift_left(132);
        assert_eq!(big_num_1.get_hex(), "00000000000000051bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4000000000000000000000000000000000");

        big_num_1.set_hex("122e962951c3").unwrap();
        big_num_1.bitshift_left(1);
        assert_eq!(big_num_1.get_hex(), "0000245d2c52a386");


        big_num_1.set_hex("ffffffffffffffff").unwrap();
        big_num_1.bitshift_left(1);
        assert_eq!(big_num_1.get_hex(), "0000000000000001fffffffffffffffe");


    }

    #[test]
    fn add_bn() {
        let mut big_num_1: BigNumber = BigNumber::new();
        let mut big_num_2: BigNumber = BigNumber::new();

        big_num_1.set_hex("36f028580bb02cc8272a9a020f4200e346e276ae664e45ee80745574e2f5ab80").unwrap();
        big_num_2.set_hex("70983d692f648185febe6d6fa607630ae68649f7e6fc45b94680096c06e4fadb").unwrap();
        let result = big_num_1.add(&big_num_2);
        assert_eq!(result.get_hex(), "a78865c13b14ae4e25e90771b54963ee2d68c0a64d4a8ba7c6f45ee0e9daa65b");

        big_num_1.set_hex("f6f028580bb02cc8272a9a020f4200e346e276ae664e45ee80745574e2f5ab80").unwrap();
        big_num_2.set_hex("f0983d692f648185febe6d6fa607630ae68649f7e6fc45b94680096c06e4fadb").unwrap();
        let result = big_num_1.add(&big_num_2);

        assert_eq!(result.get_hex(), "0000000000000001e78865c13b14ae4e25e90771b54963ee2d68c0a64d4a8ba7c6f45ee0e9daa65b");
    }

    #[test]
    fn sub_bn() {
        let mut big_num_1: BigNumber = BigNumber::new();
        let mut big_num_2: BigNumber = BigNumber::new();

        big_num_1.set_hex("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc").unwrap();
        big_num_2.set_hex("22e962951cb6cd2ce279ab0e2095825c141d48ef3ca9dabf253e38760b57fe03").unwrap();
        let result = big_num_1.sub(&big_num_2);

        assert_eq!(result.get_hex(), "10e570324e6ffdbc6b9c813dec968d9bad134bc0dbb061530934f4e59c2700b9");

        big_num_1.set_hex("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc").unwrap();
        big_num_2.set_hex("e279ab0e2095825c141d48ef3ca9dabf253e38760b57fe03").unwrap();
        let result = big_num_1.sub(&big_num_2);

        assert_eq!(result.get_hex(), "33ced2c76b26cae86b9c813dec968d9bad134bc0dbb061530934f4e59c2700b9");
    }

    #[test]
    fn modulo_bn() {
        let mut big_num_1: BigNumber = BigNumber::new();
        let mut big_num_2: BigNumber = BigNumber::new();

        big_num_1.set_hex("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc").unwrap();
        big_num_2.set_hex("122e962951c3").unwrap();
        let result = big_num_1.modulo(&big_num_2);

        assert_eq!(result.get_hex(), "0000105fdb36dd7d");

        big_num_1.set_hex("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc").unwrap();
        big_num_2.set_hex("122e962951c3123127398172").unwrap();
        let result = big_num_1.modulo(&big_num_2);

        assert_eq!(result.get_hex(), "000000000f05056a0c19079c4c7eb0e4");

    }
}