pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(&left, &right, arr);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], arr: &mut [T]) {
    let (mut left_idx, mut right_idx, mut arr_idx) = (0, 0, 0);

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            arr[arr_idx] = left[left_idx].clone();
            left_idx += 1;
        } else {
            arr[arr_idx] = right[right_idx].clone();
            right_idx += 1;
        }
        arr_idx += 1;
    }

    while left_idx < left.len() {
        arr[arr_idx] = left[left_idx].clone();
        left_idx += 1;
        arr_idx += 1;
    }

    while right_idx < right.len() {
        arr[arr_idx] = right[right_idx].clone();
        right_idx += 1;
        arr_idx += 1;
    }
}
