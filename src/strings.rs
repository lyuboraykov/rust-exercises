use ::arrays::most_common;


pub fn most_common_word(text: &str) -> (&str, u32) {
    let words = text.split(' ');
    most_common(words)
}
