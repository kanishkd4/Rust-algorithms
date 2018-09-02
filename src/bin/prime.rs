extern crate algorithms;
use algorithms::prime_number_generator;


fn main() {
    const N: usize = 1000_;
    let mut n_array: [u64; N] = [1; N];
    n_array[0] = 0;
    n_array[1] = 0;
    let n = N as f32;
    let n_root = n.sqrt().round() as usize;
    let n = n as usize;

    for i in 2..=n_root {
        if n_array[i] == 1 {
            let j_iter = i.pow(2)..n;
            for j in j_iter.into_iter().step_by(i) {
                n_array[j] = 0;
            }
        }
    }
    let mut primes = Vec::new();
    let upper_limit = n as u32;
    for i in 0..upper_limit {
        if n_array[i as usize] == 1 {
            primes.push(i);
        }
    }
    println!("Primes up to {}: {:?}", n, primes.len());


}