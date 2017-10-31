use ::arrays::most_common;


pub fn most_common_word(text: &str) -> (&str, u32) {
    let words = text.split(' ');
    most_common(words)
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_common_word() {
        let (word, count) = most_common_word("this is a test of the test");
        assert_eq!(word, "test");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_longest() {
        let short = "a";
        let long = "ab";

        let longest = longest(&short, &long);
        assert_eq!(longest, long);
    }
}
