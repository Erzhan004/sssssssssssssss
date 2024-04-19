pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot = &arr[arr.len() / 2];
    let (mut i, mut j) = (0, arr.len() - 1);
    while i <= j {
        while arr[i] < *pivot {
            i += 1;
        }
        while arr[j] > *pivot {
            j -= 1;
        }
        if i <= j {
            arr.swap(i, j);
            i += 1;
            if j > 0 { j -= 1; }
        }
    }
    j
}
