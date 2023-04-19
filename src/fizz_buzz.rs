pub fn calculation() {
    use std::ops::Rem;
    fn fz<T>(x: T, a: T, b: T, zero: T) -> String
    where
        T: Rem<T, Output = T> + Eq + Copy + ToString,
    {
        match (x % a == zero, x % b == zero) {
            (true, true) => format!("FizzBuzz"),
            (true, _) => format!("Fizz"),
            (_, true) => format!("Buzz"),
            _ => x.to_string(),
        }
    }

    (1..=100)
        .map(|x: u32| fz(x, 3, 5, 0))
        .for_each(|x| println!("{}!", x));
}
