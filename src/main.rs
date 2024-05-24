use sort_search::part1_bubble_sort::run_bubble_sort;
use sort_search::part2_quicksort::run_quicksort;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(s) if s.eq("bubble") => run_bubble_sort(),
        Some(s) if s.eq("quick") => run_quicksort(),
        None | _ => println!(" ** provide one of: bubble, quick, ..."),
    }
}
