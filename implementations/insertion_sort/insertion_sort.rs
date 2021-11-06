use std::cmp::{Ord, Ordering};
use std::vec::Vec;

/// Insertion Sort (Out Of Place)
/// This function takes in a vector of objects that have the Ord and Copy traits and returns a vector of the same type.
/// First, we test to see if the vector has a length less than two. (Hence, length is an element of {0, 1} and does not need to be sorted.)
/// If the vector satifies this condition, there is no need to proceed with sorting and we can simply return the original vector.
/// If the vector does not satisfy this condition, we continue with the program. We create a new vector with the same capacity as the input vector.
/// For each element in the original vector, we aim to progressively build up our list by inserting our value at the appropriate location.
/// To do this, we set a counter `n = 0`. While our new value is less than the length of our new vector 
fn insertion_sort<T: Ord + Copy>(x: Vec<T>) -> Vec<T> {
    if x.len() < 2 { return x; }
    let mut ret: Vec<T> = Vec::with_capacity(x.len());
    for i in x {
        let mut n = 0;
        while n < ret.len() && i.cmp(&ret[n]) != Ordering::Less { n += 1; }
        ret.insert(n, i);
    }
    ret
}

fn insertion_sort_inplace<T: Ord>(x: &mut Vec<T>) {
    if x.len() < 2 { return; }
    for i in 0..x.len() {
        let n = x.pop().unwrap();
        let mut j = 0;
        while j < i && n.cmp(&x[j]) != Ordering::Less { j += 1; }
        x.insert(j, n);
    }
}

fn verify_sort<T: Ord>(x: &Vec<T>) {
    if x.len() == 0 { return; }
    for i in 0..(x.len() - 1) {
        assert_ne!(x[i].cmp(&x[i + 1]), Ordering::Greater);
    }
}

fn main() {
    println!("Testing in-place insertion sort");
    {
        let vecs = vec![
            vec![],
            vec![1,2,3,4],
            vec![4,3,2,1],
            vec![5,5,4,5,7,3,2,1],
            vec![1],
            vec![1,2]
        ];

        for mut v in vecs {
            insertion_sort_inplace(&mut v);
            verify_sort(&v);
        }
    }
    println!("Testing passed!");

    println!("Testing out-of-place insertion sort");
    {
        let vecs = vec![
            vec![],
            vec![1,2,3,4],
            vec![4,3,2,1],
            vec![5,5,4,5,7,3,2,1],
            vec![1],
            vec![1,2]
        ];

        for mut v in vecs {
            v = insertion_sort(v);
            verify_sort(&v);
        }
    }
    println!("Testing passed!");

}