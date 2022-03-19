use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut chars: Chars,
    }

    chars.sort();
    let ans: String = chars.iter().collect::<String>();
    println!("{}", ans);
}
