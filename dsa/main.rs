fn binary_recursion(arr: &[i32], tar: i32, lo: i32, hi: i32) -> Option<i32> {
    if lo > hi {
        return None;
    }

    let mid = lo + (hi - lo) / 2

    match arr[mid].cmp(&tar) {
        std::cmp::Less => binary_recursion(arr, tar, mid + 1, hi),
        std::cmp::Greater => binary_recursion(arr, tar, lo, mid - 1),
        std::cmp::Equal => Some(mid),
    }
}

fn binary_recursion(arr, tar) -> Option<i32> {
    if arr.empty() {
        None
    } else {
        binary_recursion(arr, tar, 0, arr.len() - 1)
    }
}

