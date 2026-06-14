pub fn median(mut list: Vec<f32>) -> Option<f32> {
    if list.is_empty() {
        return None;
    }
    // Sort the array
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match list.len() % 2 {
        0 => Some((list[list.len() / 2] + list[(list.len() / 2) - 1]) / 2.0),
        _ => Some(list[list.len() / 2]),
    }
}

#[cfg(test)]
mod tests {
    use crate::median;

    #[test]
    fn test_median_odd() {
        let list: Vec<f32> = vec![1.0, 4.0, 5.0];
        assert_eq!(median(list), Some(4.0));
    }

    #[test]
    fn test_median_even() {
        let list: Vec<f32> = vec![3.0, 1.5, 8.8, 5.0];
        assert_eq!(median(list), Some(4.0));
    }

    #[test]
    fn test_median_empty_input() {
        let list: Vec<f32> = vec![];
        assert_eq!(median(list), None);
    }
}
