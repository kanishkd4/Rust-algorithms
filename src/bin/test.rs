extern crate algorithms;
use algorithms::insertion_sort;

fn main() {
    insertion_sort::test_print();
    insertion_sort::test_string_comparison();
    let vec: Vec<&str> = vec!["10", "2", "3"];
    insertion_sort::insertion_sort(vec);
}