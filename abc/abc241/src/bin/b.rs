use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        b: [usize; m],
    }

    let mut idx: usize = 0;
    let mut is_found: bool = false;
    let mut ans: bool = true;

    for b_i in b.iter() {
        for (j, a_j) in a.iter().enumerate() {
            // 見つけた！
            if b_i == a_j {
                idx = j;
                is_found = true
            }
        }
        // 食べたい長さのパスタのなかった時点で終了
        if !is_found {
            ans = false;
            break;
        // パスタがあったら食べて、次の日に
        } else {
            a[idx] = 0; // 食べる
            is_found = false;
        }
    }

    if ans {
        println!("Yes")
    } else {
        println!("No")
    }
}
