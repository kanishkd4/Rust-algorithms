use std::vec::Vec;

pub fn test_print() {
    println!("Called algorithms.insertion_sort");
}

pub fn test_string_comparison() {
    if "Bell" < "Mark" {
        println!("Working");
    }
    else {
        println!("Not working");
    }
}

pub fn insertion_sort(vec: Vec<i32>) {
    println!("Original vector: {:?}", vec);
    let mut new_vector: Vec<i32> = Vec::new();
    for (i, item) in vec.iter().enumerate() {
        println!("i: {}", i);
        if i == 0 {
            new_vector.push(*item);
        }
        else {
            for j in 0..new_vector.len() {
                println!("j: {}", j);
                if item < &new_vector[j] {
                    // split the vector into at j and add item to the first one
                    // combine the vectors to new_vector again
                }
            }
        }
    }
    
    println!("new_vector: {:?}", new_vector);
    let mut new_vec_test = &vec[..2];
    println!("new vec test: {:?}", new_vec_test);
    let vec = new_vec_test.extend_from_slice(&mut new_vector);
    println!("pushing one vector into another: {:?}", vec);
}