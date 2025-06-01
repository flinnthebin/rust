#[allow(dead_code)]
fn fizzbuzz() {
    (1..=100) // *range* generates an inclusive set
        .map(|i| match (i % 3, i % 5) {
            // *mapping* transforms each element of set
            (0, 0) => "FizzBuzz".to_string(), // mapping rule 1 (modulo result i % 3 && i % 5)
            (0, _) => "Fizz".to_string(),     // mapping rule 2 (modulo result i % 3)
            (_, 0) => "Buzz".to_string(),     // mapping rule 3 (modulo result i % 5)
            _ => i.to_string(),               // default print as string
        })
        .for_each(|output| println!("{}", output)); // *for_each* print each transformed value
}

#[cfg(test)]
mod test {
    use super::fizzbuzz;
    #[test]
    fn basic() {
        fizzbuzz();
    }
}
