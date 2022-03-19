use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }

    let ans: f64 = if x <= a {
        1.0
    } else if x <= b {
        c as f64 / (b - a) as f64
    } else {
        0.0
    };

    println!("{:.12}", ans)
}
