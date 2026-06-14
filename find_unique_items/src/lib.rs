use std::collections::HashSet;

//pub fn unique(mut list: Vec<i32>) -> Vec<i32> {
//    list.sort_unstable();
//    list.dedup();
//    list
//}

// Extra Credit
//
// Generics:
// Use Generics to accept Vec<T>
// where T is type that implements Ord.
//
// Retain order:
// Return elements in their original order.
pub fn unique<T: Ord + std::hash::Hash + Copy>(list: Vec<T>) -> Vec<T> {
    let mut new_list = vec![];
    let mut set = HashSet::new();
    for x in list {
        if set.insert(x) {
            new_list.push(x);
        }
    }
    new_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list: Vec<i32> = vec![];
        assert_eq!(unique(list), vec![]);
    }

    #[test]
    fn test_sorted_list() {
        let list = vec![1, 4, 5];
        assert_eq!(unique(list), vec![1, 4, 5]);
    }

    #[test]
    fn test_unsorted_list() {
        let list = vec![4, 1, 5];
        assert_eq!(unique(list), vec![4, 1, 5]);
    }

    #[test]
    fn test_sorted_list_with_duplicates() {
        let list = vec![1, 1, 3];
        assert_eq!(unique(list), vec![1, 3]);
    }

    #[test]
    fn test_unsorted_list_with_duplicates() {
        let list = vec![3, 1, 1];
        assert_eq!(unique(list), vec![3, 1]);
    }
}
