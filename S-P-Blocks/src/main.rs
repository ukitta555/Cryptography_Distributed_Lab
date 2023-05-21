// extern crate hex;
// use hex::decode;

mod s_box;
mod tests;
mod p_box;



fn main() {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    // transform
    let converted = s_box::transform(&data);
    let pre_image = s_box::inverse_transform(&converted);
    assert_eq!(pre_image, vec![1, 2, 3, 4, 5]);

    let data: u8 = 123;
    let key: Vec<usize> = vec![4, 2, 1, 3, 0, 7, 6, 5];
    let converted = p_box::transform(data, &key);
    let pre_image = p_box::inverse_transform(converted, &key);
    assert_eq!(pre_image, 123);
    println!("All good, the code didn't panic!");
}
