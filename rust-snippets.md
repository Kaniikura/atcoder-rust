# rustで競プロをするためのスニペット集
## 入出力系
### 入力
`proconio::input` マクロが便利
```rust
use proconio::input;
use proconio::marker::{Chars,Bytes};


fn main() {
    input! {
        n: usize, // 符号なし整数型
        m: usize, // 符号なし整数型
        a: [[i32; n]; m], // 2次元配列 (n, m)
        string: String, // 文字列
        chars: Chars, // char型配列
        bytes: Bytes, // バイト列
    }
    
    // なんらかのコード
}
```

### 出力
桁数を指定
```rust
let x: f64 = 0.123456789012345;
println!("{:.10}", x); //精度10桁で出力 -> 0.123456789 

```

## データ構造


## アルゴリズム
### 素数判定
```rust
fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let mut result = true;

    let s_root = (n as f64).sqrt() as usize;
    for i in 2..=s_root {
        if n % i == 0 {
            result = false;
            break;
        }
    }
    return result;
}
```

### 約数の列挙
```rust
fn make_divisors(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let s_root = (n as f64).sqrt() as usize;
    for i in 1..=s_root {
        if n % i == 0 {
            res.push(i);
            let div = n / i;
            if div != i {
                res.push(div);
            }
        }
    }
    res.sort();
    return res;
}
```

### 幅優先探索 (bfs)
```rust
use std::collections::VecDeque;


pub fn bfs(from: usize, nodes: &Vec<Vec<usize>>) -> Vec<isize> {
    let node_num = nodes.len();
    let mut todo = VecDeque::new();
    let mut visited = vec![-1; node_num];

    todo.push_back(from);
    visited[from] = 0;

    while !todo.is_empty() {
        let visit_node = todo.pop_front().unwrap();
        let push_nodes = &nodes[visit_node];
        for n in push_nodes {
            if visited[*n] != -1 {
                continue;
            }
            visited[*n] = visited[visit_node] + 1;
            todo.push_back(*n);
        }
    }
    return visited;
}
```
