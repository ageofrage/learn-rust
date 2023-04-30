pub fn calculation() {
    struct FizzBuzz<T> {
        div_a: T,
        div_b: T,
        zero: T,
    }

    impl<T> FizzBuzz<T> {
        fn new(div_a: T, div_b: T, zero: T) -> Self {
            FizzBuzz { div_a, div_b, zero }
        }
    }

    trait ToFzStr<T> {
        fn to_str(&self, x: T) -> String;
    }

    fn common_fizz_buzz<T>(x: T, div_a: T, div_b: T, zero: T) -> String
    where
        T: std::ops::Rem<T, Output = T> + Eq + Copy + ToString,
    {
        {
            match (x % div_a == zero, x % div_b == zero) {
                (true, true) => format!("FizzBuzz"),
                (true, _) => format!("Fizz"),
                (_, true) => format!("Buzz"),
                _ => x.to_string(),
            }
        }
    }

    impl ToFzStr<u32> for FizzBuzz<u32> {
        fn to_str(&self, x: u32) -> String {
            common_fizz_buzz(x, self.div_a, self.div_b, self.zero)
        }
    }

    impl ToFzStr<i64> for FizzBuzz<i64> {
        fn to_str(&self, x: i64) -> String {
            common_fizz_buzz(x, self.div_a, self.div_b, self.zero)
        }
    }

    (1..=100)
        .map(|x: u32| FizzBuzz::new(3, 5, 0).to_str(x))
        .for_each(|x| println!("{}", x))
}
