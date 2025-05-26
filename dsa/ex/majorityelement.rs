fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = nums[0];
    let mut count = 0;

    for num in nums {
        if count == 0 {
            candidate = num;
        }
        if candidate == num {
            count += 1
        } else {
            count -= 1
        }
    }
    candidate
}

fn main() {
    let nums = vec![2, 2, 1, 1, 3, 3, 2, 2, 2, 2, 5];
    println!("{}", majority_element(nums));
}
