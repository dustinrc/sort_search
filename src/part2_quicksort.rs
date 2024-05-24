use crate::run_sort;

// quick sort with lomuto partition scheme
// fn quicksort(vec: &mut Vec<i32>) {}
fn _quicksort(A: &mut [i32]) {
    // for purposes of lesson, create hi and lo (normally unnecessary)
    let hi = A.len().saturating_sub(1);
    let lo = 0;

    // would need these checks if array instead of slice
    // if lo >= hi || lo < 0 {
    if hi == 0 {
        return;
    }

    let p = partition(A);

    _quicksort(&mut A[lo..p]);
    _quicksort(&mut A[p + 1..=hi]);
}

fn partition(A: &mut [i32]) -> usize {
    // for purposes of lesson, create hi and lo (normally unnecessary)
    let hi = A.len() - 1;
    let lo = 0;

    // choose last element as pivot
    let pivot = A[hi];
    // temporary pivot index
    let mut i = lo;

    for j in (lo..hi) {
        if A[j] <= pivot {
            A.swap(i, j);
            i += 1;
        }
    }
    A.swap(i, hi);
    i
}

pub fn quicksort(vec: &mut Vec<i32>) {
    _quicksort(vec);
}

// Run in main
pub fn run_quicksort() {
    run_sort(quicksort);
}
