fn binary_recursion(arr: &[i32], tar: i32, lo: usize, hi: usize) -> Option<usize> {
    if lo > hi {
        return None;
    }

    let mid = lo + (hi - lo) / 2; // find mid
    match arr[mid].cmp(&tar) {
        std::cmp::Ordering::Less => binary_recursion(arr, tar, mid + 1, hi),
        std::cmp::Ordering::Greater => binary_recursion(arr, tar, lo, mid - 1),
        std::cmp::Ordering::Equal => Some(mid),
    }
}

fn binary_search(arr: &[i32], tar: i32) -> Option<usize> {
    if arr.is_empty() {
        None
    } else {
        binary_recursion(arr, tar, 0, arr.len() - 1)
    }
}

fn main() {
    let sorted_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;

    match binary_search(&sorted_array, target) {
        Some(index) => println!("Found target {} at index {}", target, index),
        None => println!("Target {} not found in the array", target),
    }
}
