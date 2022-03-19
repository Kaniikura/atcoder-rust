use proconio::input;

fn main() {
    input! {
        n: i32,
        a_list: [i32; n],
        b_list: [i32; n],
    };

    let mut ans: [i32; 2] = [0, 0];
    for (i, a) in a_list.iter().enumerate() {
        for (j, b) in b_list.iter().enumerate() {
            if a == b {
                if i == j {
                    ans[0] += 1;
                } else {
                    ans[1] += 1;
                }
            }
        }
    }
    println!("{}", ans[0]);
    println!("{}", ans[1]);
}
