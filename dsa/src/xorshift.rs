// random number generator
// without requiring crates

#[allow(dead_code)]
struct XorShift {
    state: u32,
}
#[allow(dead_code)]
impl XorShift {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
}

#[cfg(test)]
mod test {
    use super::XorShift;
    use std::time::{SystemTime, UNIX_EPOCH};
    #[test]
    fn basic() {
        let seed_u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time")
            .as_nanos();
        let seed_u32 = (seed_u128 & 0xFFFFFFFF) as u32;
        let mut y = XorShift::new(seed_u32);
        let z = y.next();
        println!(
            "u128 seed: {}, u32 seed: {}, random: {}",
            seed_u128, seed_u32, z
        );
    }
}
