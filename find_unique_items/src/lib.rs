// Extra Credit
//
// Generics:
// Use Generics to accept Vec<T>
// where T is type that implements Ord.
//
// Retain order:
// Return elements in their original order.
pub fn unique<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort();
    list.dedup();
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_with_no_duplicates() {
        let list = vec![1, 4, 5];
        assert_eq!(unique(list), vec![1, 4, 5]);
    }

    #[test]
    fn test_list_with_duplicates() {
        let list = vec![1, 1, 3];
        assert_eq!(unique(list), vec![1, 3]);
    }
}
