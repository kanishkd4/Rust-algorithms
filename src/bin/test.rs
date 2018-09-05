extern crate algorithms;
use algorithms::insertion_sort;
use std::vec::Vec;

fn main() {
    // insertion_sort::test_print();
    // insertion_sort::test_string_comparison();
    let mut vec = vec![10, 2, 3]; // std::vec::Vec
    insertion_sort::insertion_sort_int(&mut vec);
}