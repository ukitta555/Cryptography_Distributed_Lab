#[cfg(test)]
mod tests {
    use crate::s_box;
    use crate::p_box;

    #[test]
    fn should_be_identical_after_direct_and_reverse_s_box() {
        // initialize data
        let data: Vec<u8> = vec![1, 2, 3, 4, 5];
        // transform
        let converted = s_box::transform(&data);
        let pre_image = s_box::inverse_transform(&converted);
        assert_eq!(pre_image, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_be_identical_after_direct_and_reverse_p_box() {
        let data: u8 = 123;
        let key: Vec<usize> = vec![4, 2, 1, 3, 0, 6, 5, 7];
        let converted = p_box::transform(data, &key);
        let pre_image = p_box::inverse_transform(converted, &key);
        assert_eq!(pre_image, 123);
    }
}