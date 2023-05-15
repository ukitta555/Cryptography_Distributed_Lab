mod test;
mod big_number;

use crate::big_number::BigNumber;

// This is my first (somewhat) large project using this language; please don't judge harshly!
fn main() {
    let mut big_num: BigNumber = BigNumber::new();
    big_num.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4");
    big_num.bitshift_right(132);
}
