use itertools::Itertools;
use proconio::input;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::PI;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[i32; n]; m],
        string: String,
        chars: proconio::marker::Chars,
        bytes: proconio::marker::Bytes,
    }
}

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

fn print_typename<T>(_: &T) {
    println!("type=<{}>", std::any::type_name::<T>());
}
