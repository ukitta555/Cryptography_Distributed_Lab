pub fn transform(data: &Vec<u8>, key: &Vec<usize>) -> Vec<u8> {
    assert_eq!(data.len(), key.len());
    let mut result: Vec<u8> = vec![0; data.len()];
    for i in 0..result.len() {
        // for each index of result, pick an entry from the initial data using the permutation key
        // A B C D E - data
        // 5 3 2 4 1 - key
        // E C B D A - result
        result[i] = data[key[i]];
    }
    result
}

pub fn inverse_transform(data: &Vec<u8>, key: &Vec<usize>) -> Vec<u8> {
    assert_eq!(data.len(), key.len());
    let mut result: Vec<u8> = vec![0; data.len()];
    for i in 0..key.len() {
        // switch indices around for inverse opration (note that the keys should be identical for an inverse to exist!)
        // E C B D A - permuted data
        // 5 3 2 4 1 - key
        // A B C D E - result
        result[key[i]] = data[i];
    }
    result
}