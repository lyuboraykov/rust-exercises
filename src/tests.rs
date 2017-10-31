use arrays::most_common;
use strings::most_common_word;

#[test]
fn test_most_common_word() {
    let (word, count) = most_common_word("this is a test of the test");
    assert_eq!(word, "test");
    assert_eq!(count, 2);
}

#[test]
fn test_most_common() {
    let test_arr = [1, 2, 3, 4, 1];
    let (most_common, count) = most_common(test_arr.iter());
    assert_eq!(*most_common, 1);
    assert_eq!(count, 2);
}
