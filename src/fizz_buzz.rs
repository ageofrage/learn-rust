pub fn calculation() {
    let fz = |x: i32| match (x % 3, x % 5) {
        (0, 0) => format!("FizzBuzz"),
        (0, _) => format!("Fizz"),
        (_, 0) => format!("Buzz"),
        _ => x.to_string(),
    };
    (1..=100).map(fz).for_each(|s| println!("{}", s));
}
