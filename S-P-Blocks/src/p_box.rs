pub fn transform(data: u8, key: &Vec<usize>) -> u8 {
    assert_eq!(key.len(), 8);
    let mut result: u8 = 0;
    for i in 0..8 {
        // for each index of result, pick an entry from the initial data using the permutation key
        // A B C D E F G H - data
        // 4 2 1 3 0 6 5 7 - key
        // E C B D A G F H - result
        // fetch the bit, make it the LSB, push it to the correct place
        result |= ((data & (1 << (7 - i))) >> (7 - i)) << (7 - key[i]);
    }
    result
}

pub fn inverse_transform(data: u8, key: &Vec<usize>) -> u8 {
    assert_eq!(key.len(), 8);
    let mut result: u8 = 0;
    for i in 0..8 {
        // do the same for inverse opration (note that the keys should be identical for an inverse to be correct!)
        // E C B D A G F H - permuted data
        // 4 2 1 3 0 6 5 7 - key
        // A B C D E F G H - result
        // fetch the bit, make it the LSB, push it to the correct place
        result |= ((data & (1 << (7 - i))) >> (7 - i)) << (7 - key[i]);
    }
    result
}