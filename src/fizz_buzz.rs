pub fn calculation() {
    let thinking: &str = "...";
    for i in 1..=30 {
        if i % 15 == 0 {
            println!("{}? {} FizzBuzz", i, thinking);
        } else if i % 3 == 0 {
            println!("{}! Fizz", i);
        } else if i % 5 == 0 {
            println!("{}! Buzz", i);
        } else {
            println!("{}", i);
        }
    }
}
