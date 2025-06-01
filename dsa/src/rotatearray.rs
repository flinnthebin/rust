#[allow(dead_code)]
fn rotate(nums: &mut [i32], k: i32) {
    let len = nums.len();
    if len == 0 {
        return;
    }
    let shift = (k as usize) % len;
    nums.rotate_right(shift);
}

#[cfg(test)]
mod test {
    #[test]
    fn example() {
        use super::rotate;
        let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, [5, 6, 7, 1, 2, 3, 4])
    }
}
