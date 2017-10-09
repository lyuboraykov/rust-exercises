fn main() {
    const N: u64 = 400;

    let mut first: u64 = 0;
    let mut second: u64 = 1;

    for _ in 1..N {
        let temp: u64 = second;
        second = first + second;
        first = temp;
    }

    println!("Result is {}", second);
}
