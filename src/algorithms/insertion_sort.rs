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

// was attempting to subset the vector but cannot figure out applying extend_from_slice to &[i32]
// pub fn insertion_sort_int(vec: Vec<i32>) {
//     println!("Original vector: {:?}", vec);
//     let mut new_vector: Vec<i32> = Vec::new();
//     for (i, item) in vec.iter().enumerate() {
//         println!("i: {}", i);
//         if i == 0 {
//             new_vector.push(*item);
//         }
//         else {
//             for j in 0..new_vector.len() {
//                 println!("j: {}", j);
//                 if item < &new_vector[j] {
//                     // split the vector into at j and add item to the first one
//                     // combine the vectors to new_vector again
//                     let mut new_vec1 = &new_vector[0..j];
//                     let new_vec2 = &new_vector[j..];
//                     println!("new vec 1: {:?}", new_vec1);
//                     println!("new vec 2: {:?}", new_vec2);
//                     let mut new_vector = new_vec1.extend_from_slice(new_vec2);
//                 }
//             }
//         }
//     }
    
//     println!("new_vector: {:?}", new_vector);
//     let mut new_vec_test = &vec[..2];
//     println!("new vec test: {:?}", new_vec_test);
//     // let vec = new_vec_test.extend_from_slice(&mut new_vector);
//     // println!("pushing one vector into another: {:?}", vec);
// }

pub fn insertion_sort_int(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    println!("Original vector: {:?}", vec);
    for i in 0..vec.len() {
        for j in (0..i).rev() {
            if vec[j] >= vec[j + 1] {
                vec.swap(j, j + 1);
            } else {
                break
            }
        }
    }
    println!("modified vector: {:?}", vec);
    return vec;
}