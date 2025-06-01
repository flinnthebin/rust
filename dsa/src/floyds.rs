#[allow(dead_code)]
fn tortoise_hare<F>(mut f: F, x0: usize) -> Option<(usize, usize)>
where
    F: FnMut(usize) -> usize,
{
    // Find collison
    let mut tortoise = f(x0);
    let mut hare = f(x0);
    hare = f(hare); // let hare = f(f(x0)
    while tortoise != hare {
        tortoise = f(tortoise);
        hare = f(hare);
        hare = f(hare); // let hare = f(f(x0)
    }

    // Find mu (start index)
    let mut mu = 0;
    tortoise = x0;
    while tortoise != hare {
        tortoise = f(tortoise);
        hare = f(hare);
        mu += 1;
    }

    // Find lambda (cycle length)
    let mut lambda = 1;
    hare = f(tortoise);
    while tortoise != hare {
        hare = f(hare);
        lambda += 1;
    }

    Some((mu, lambda))
}

#[cfg(test)]
mod test {
    use super::tortoise_hare;
    #[test]
    fn basic() {
        let f = |x| (x * x + 1) % 255;
        let start = 2;
        if let Some((mu, lambda)) = tortoise_hare(f, start) {
            assert_eq!(mu, 0);
            assert_eq!(lambda, 6);
        }
    }
}
