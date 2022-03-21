use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let a_set = HashSet::<i32>::from_iter(a.iter().cloned());
    println!("{}", a_set.len());
}
