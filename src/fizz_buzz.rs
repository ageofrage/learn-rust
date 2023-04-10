pub fn calculation() {
    fn fz(x: i32) -> String {
        match (x % 3, x % 5) {
            (0, 0) => format!("FizzBuzz"),
            (0, _) => format!("Fizz"),
            (_, 0) => format!("Buzz"),
            _ => x.to_string(),
        }
    }

    let res = (1..=100).map(fz).collect::<Vec<_>>().join("!\n");

    println!("{}", res);
}
