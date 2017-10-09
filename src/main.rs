fn main() {
    let mut s = String::from("this is a string");
    takes_a_reference(&mut s);

    println!("{}", s);
}

fn takes_a_reference(s: &mut String) {
    s.push_str(" this is a pushed string");
}
