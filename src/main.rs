extern crate service_utils;

use service_utils::strings::most_common_word;


fn main() {
    let (word, count) = most_common_word("what is the most common word in this doc? what");

    println!("{}-{}", word, count);
}
