use proconio::input;

fn main() {
    input! {
        a: [usize; 10],
    }

    let num_push: usize = 3;
    let mut cur: usize = 0;
    for _ in 0..num_push {
        cur = a[cur];
    }
    println!("{}", cur);
}
