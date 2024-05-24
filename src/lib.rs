pub(crate) mod prng;
pub(crate) mod prompt;
pub(crate) mod vector;

pub mod part1_bubble_sort;

use crate::{
    prompt::get_i32,
    vector::{check_sorted, make_random_vec, print_vec},
};

// Run algorithm main.
pub fn run_sort(f: fn(&mut Vec<i32>)) {
    let num_items = get_i32("How many items? ");
    let num_max = get_i32("What is the largest number? ");

    let mut vec = make_random_vec(num_items, num_max);
    print_vec(&vec, num_items);
    f(&mut vec);

    print_vec(&vec, num_items);
    check_sorted(&vec);
}
