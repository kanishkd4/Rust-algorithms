extern crate algorithms;
use algorithms::insertion_sort;
use std::vec::Vec;

fn main() {
    // insertion_sort::test_print();
    // insertion_sort::test_string_comparison();
    // let vec: Vec<i32> = vec![10, 2, 3]; // std::vec::Vec
    // insertion_sort::insertion_sort(vec);
    // let mut vec = vec![1];
    // vec.extend_from_slice(&[2, 3, 4]);
    // assert_eq!(vec, [1, 2, 3, 4]);

    let mut vec = vec![1];
    let vec1 = [2, 3, 4];
    vec.extend_from_slice(&vec1);
    assert_eq!(vec, [1, 2, 3, 4]);
    println!("vec: {:?}", vec);
}