use rand::prelude::*;
use std::time::Instant;

mod bubble_sort;
mod merge_sort;
mod parallel_merge_sort;
mod quick_sort;

fn main() {
    let total_size = 120;
    // create random array
    // let mut array1: Vec<u32> = (0..total_size).collect();
    // array1.shuffle(&mut thread_rng());
    // println!("Array1: {:?}", array1);
    //
    // quick_sort::sort(&mut array1);
    // println!("Quicksort: {:?}", array1);
    //
    // let mut array2: Vec<u32> = (0..total_size).collect();
    // array2.shuffle(&mut thread_rng());
    // println!("Array2: {:?}", array2);
    //
    // bubble_sort::sort(&mut array2);
    // println!("BubbleSort: {:?}", array2);
    //
    let mut array3: Vec<u32> = (0..total_size).collect();
    array3.shuffle(&mut thread_rng());
    // println!("Array3: {:?}", array3);
    let now = Instant::now();
    {
        merge_sort::sort(&mut array3);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    // println!("Merge Sort:{:?}", array3);

    let mut array1: Vec<u32> = (0..total_size).collect();

    array1.shuffle(&mut thread_rng());
    // println!("Array1: {:?}", array1);

    let now = Instant::now();
    {
        parallel_merge_sort::sort(&mut array1);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
