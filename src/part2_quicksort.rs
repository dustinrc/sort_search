use crate::run_sort;

// quick sort with lomuto partition scheme
// fn quicksort(vec: &mut Vec<i32>) {}
fn _quicksort(arr: &mut [i32]) {
    // for purposes of lesson, create hi and lo (normally unnecessary)
    let hi = arr.len().saturating_sub(1);
    let lo = 0;

    // would need these checks if array instead of slice
    // if lo >= hi || lo < 0 {
    if hi == 0 {
        return;
    }

    let p = partition(arr);

    _quicksort(&mut arr[lo..p]);
    _quicksort(&mut arr[p + 1..=hi]);
}

fn partition(arr: &mut [i32]) -> usize {
    // for purposes of lesson, create hi and lo (normally unnecessary)
    let hi = arr.len() - 1;
    let lo = 0;

    // choose last element as pivot
    let pivot = arr[hi];
    // temporary pivot index
    let mut i = lo;

    for j in lo..hi {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, hi);
    i
}

pub fn quicksort(vec: &mut Vec<i32>) {
    _quicksort(vec);
}

// Run in main
pub fn run_quicksort() {
    run_sort(quicksort);
}
