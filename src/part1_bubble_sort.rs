use crate::run_sort;

// Use bubble sort to sort the vector.
fn bubble_sort(vec: &mut Vec<i32>) {
    let mut passes = 0;
    for x in (0..vec.len()).rev() {
        passes += 1;
        let mut swaps = 0u32;
        for y in 0..x {
            if vec[y] > vec[y + 1] {
                vec.swap(y, y + 1);
                swaps += 1;
            }
        }
        println!("swaps: {swaps}");
        if swaps == 0 {
            break;
        }
    }
    println!("passes: {passes}");
}

// Run in main.
pub fn run_bubble_sort() {
    run_sort(bubble_sort);
}
