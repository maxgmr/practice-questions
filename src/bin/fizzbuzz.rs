fn main() {
    fizz_buzz(1);
}
pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..(n + 1))
        .map(|i| match i % 3 {
            0 => match i % 5 {
                0 => String::from("FizzBuzz"),
                _ => String::from("Fizz"),
            },
            _ => match i % 5 {
                0 => String::from("Buzz"),
                _ => format!("{}", i),
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test() {
        assert_eq!(fizz_buzz(0), Vec::<String>::new());
        assert_eq!(fizz_buzz(1), vec![String::from("1")]);
        assert_eq!(
            fizz_buzz(16),
            vec![
                String::from("1"),
                String::from("2"),
                String::from("Fizz"),
                String::from("4"),
                String::from("Buzz"),
                String::from("Fizz"),
                String::from("7"),
                String::from("8"),
                String::from("Fizz"),
                String::from("Buzz"),
                String::from("11"),
                String::from("Fizz"),
                String::from("13"),
                String::from("14"),
                String::from("FizzBuzz"),
                String::from("16"),
            ]
        );
    }
}
