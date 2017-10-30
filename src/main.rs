extern crate service_utils;

use service_utils::strings::{most_common_word, longest};

fn main() {
    let (word, count) = most_common_word("this is a test for the test");
    println!("{}", longest("this is a", "this is the b"));
    println!("{} - {}", word, count);
}
