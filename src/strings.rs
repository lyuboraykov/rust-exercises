use std::collections::HashMap;

pub fn most_common_word(text: &str) -> (&str, u32) {
    let mut word_count: HashMap<&str, u32> = HashMap::new();

    let mut most_common_count = 0;
    let mut most_common_word: &str = "";

    for word in text.split(' ') {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
        if *count > most_common_count {
            most_common_count = *count;
            most_common_word = word;
        }
    }

    (most_common_word, most_common_count)
}
