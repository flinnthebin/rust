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
    (1..=100)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => i.to_string(),
        })
        .for_each(|output| println!("{}", output));
}

fn main() {
    fancybuzz();
}
