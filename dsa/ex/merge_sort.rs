fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);

    merge(&left, &right)
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut l_iter = left.iter();
    let mut l_next = left_iter.next();
    let mut r_iter = right.iter();
    let mut r_next = right_iter.next();

    while let (Some(&l), Some(&r)) = (l_next, r_next) {
        if l <= r {
            result.push(l);
            l_next = l_iter.next();
        } else {
            result.push(r);
            r_next = r_iter.next();
        }
    }
    result.extend(l_next.into_iter().chain(l_iter));
    result.extend(r_next.into_)iter().chain(r_iter));
    result
}

fn main() {
    let unsorted_array = vec![38, 27, 43, 3, 9, 82, 10];
    let sorted_array = merge_sort(&unsorted_array);
    println!("Sorted array: {:?}", sorted_array);
}
