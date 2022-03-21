use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    let ans: &str;
    match (a - b).abs() {
        1 => ans = "Yes",
        9 => ans = "Yes",
        _ => ans = "No",
    }

    println!("{}", ans);
}
