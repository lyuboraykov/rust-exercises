use std::collections::HashMap;
use std::hash::Hash;

pub fn most_common<T: Hash + Eq + Clone, I>(items: I) -> (T, u32)
    where I: Iterator<Item = T>
{
    let mut el_count: HashMap<T, u32> = HashMap::new();

    let items_list: Vec<T> = items.collect();

    let mut most_common_count = 0;
    // this would panic if items is empty though, TODO: consider returing a Result
    let mut most_common_el: &T = &items_list[0];

    for el in items_list.iter() {
        let count = el_count.entry(el.clone()).or_insert(0);
        *count += 1;
        if *count > most_common_count {
            most_common_count = *count;
            most_common_el = &el;
        }
    }

    // clone so that it's not borrowed from the original context
    (most_common_el.clone(), most_common_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_common() {
        let test_arr = [1, 2, 3, 4, 1];
        let (most_common, count) = most_common(test_arr.iter());
        assert_eq!(*most_common, 1);
        assert_eq!(count, 2);
    }
}
