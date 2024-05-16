use crate::prompt::get_i32;
use crate::vector::{make_random_vec, print_vec};

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

// Verify that the Vec is sorted.
fn check_sorted(vec: &Vec<i32>) {
    if vec.windows(2).all(|w| w[0] <= w[1]) {
        println!("{}", "The vector is sorted!")
    } else {
        println!("{}", "The vector is NOT sorted!")
    }
}

// Run in main.
pub fn run_bubble_sort() {
    let num_items = get_i32("How many items? ");
    let num_max = get_i32("What is the largest number? ");

    let mut vec = make_random_vec(num_items, num_max);
    print_vec(&vec, num_items);
    bubble_sort(&mut vec);

    print_vec(&vec, num_items);
    check_sorted(&vec);
}
