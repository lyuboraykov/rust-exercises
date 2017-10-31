use std::{thread, time};

use arrays::most_common;
use rate_limiter::{AcquireResult, RateLimiter};
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

#[test]
fn test_rate_limiter() {
    const LIMIT: u32 = 10;
    let mut limiter = RateLimiter::new(1, LIMIT);
    for _ in 0..LIMIT {
        assert_eq!(limiter.acquire(), AcquireResult::Free);
    }
    assert_eq!(limiter.acquire(), AcquireResult::Limited);
    thread::sleep(time::Duration::from_secs(1));
    assert_eq!(limiter.acquire(), AcquireResult::Free);
}
