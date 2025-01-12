fn _fizzbuzz() {
    let mut count = 1;
    while count <= 100 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("fizzbuzz")
        } else if count % 3 == 0 {
            println!("fizz")
        } else if count % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count)
        }
        count += 1;
    }
    return;
}

fn fancybuzz() {
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

fn main() {
    fancybuzz();
}
