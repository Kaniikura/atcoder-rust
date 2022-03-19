use proconio::input;

fn main() {
    input! {
        v: i32,
        a: i32,
        b: i32,
        c: i32,
    };

    let res: i32 = v % (a + b + c);

    let answer: &str = if a > res {
        "F"
    } else if a + b > res {
        "M"
    } else if a + b + c > res {
        "T"
    } else {
        ""
    };

    print!("{}", answer);
}
