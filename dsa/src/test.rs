fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        match target.cmp(&nums[mid]) {
            std::cmp::Ordering::Equal => return mid as i32,
            std::cmp::Ordering::Greater => left = mid + 1,
            std::cmp::Ordering::Less => right = mid,
        }
    }
    -1
}
