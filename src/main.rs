struct UserAttribute {
    key: String,
    value: String
}

fn main() {
    let s = String::from("this is a string");
    let user = UserAttribute {
        key: String::from("this is a key"),
        value: String::from("this is a value")
    };

    println!("user attribute: {}, {}", user.key, user.value);

    println!("{}", get_first_n_words(&s, 2));
}

fn get_first_n_words(s: &str, n: u32) -> &str {
    let bytes = s.as_bytes();
    let mut count = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            count += 1;
        }
        if count == n {
            return &s[..i];
        }
    }
    &s
}
