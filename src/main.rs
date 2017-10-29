extern crate service_utils;

use service_utils::strings::most_common_word;

fn main() {
    let (word, count) = most_common_word("this is a test for the test");

    println!("{} - {}", word, count);
}
