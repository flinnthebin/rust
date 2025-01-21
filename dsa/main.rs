fn binary_recursion(arr: &[i32], tar: i32, lo: i32, hi: i32) -> Option<i32> {
    if lo > hi {
        None
    }

    let mid = lo + (hi - lo) / 2;

    match arr[mid].cmp(&tar) {
        std::cmp::Ordering::Less => binary_recursion(arr, tar, mid + 1, hi),
        std::cmp::Ordering::Greater => binary_recursion(arr, tar, 0, mid - 1),
        std::cmp::Ordering::Equal => Some(mid),
    }
}

fn binary_search(arr: &[i32], tar: i32) -> Option<i32> {
    if arr.is_empty() {
        None
    } else {
        binary_recursion(arr, tar, 0, arr.len() - 1)
    }
}

fn main() {}
