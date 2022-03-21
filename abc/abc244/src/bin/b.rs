use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        t: Chars,
    }

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut r_count: usize = 0;
    for i in 0..n {
        if t[i] == 'S' {
            match r_count % 4 {
                0 => x += 1, // 東に進む
                1 => y -= 1, // 南に進む
                2 => x -= 1, // 西に進む
                _ => y += 1, // 北に進む
            }
        } else {
            r_count += 1; // 時計回りに90°方向転換
        }
    }

    println!("{} {}", x, y);
}
