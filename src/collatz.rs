// calculate the lengths of shortened ((3n+1)/2 instead of (3n+1)) paths of Collatz conjecture
// between 2 and SIZE, giving a collatz_lenghts vector

// later, solution to https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=36

use std::{io::prelude::*, io};

const SIZE: usize = 10001;

fn calculate_collatz_length(n: usize, collatz_lengths: &mut Vec<usize>) {
    let mut x = n;
    let mut current_seq: Vec<usize> = vec![];

    while collatz_lengths[x] == 0 {
        current_seq.push(x);
        if x % 2 == 0 {
            x /= 2;
        } else {
            x = 3 * x + 1;
        }

        if collatz_lengths.len() < x + 1 {
            collatz_lengths.resize(x + 1, 0);
        }
    }

    for (idx, &num) in (current_seq).iter().rev().enumerate() {
        collatz_lengths[num] = collatz_lengths[x] + idx + 1;
    }
}

fn main() {
    let mut collatz_lengths: Vec<usize> = vec![0; SIZE];
    collatz_lengths[1] = 1;
    for n in 2..SIZE {
        calculate_collatz_length(n, &mut collatz_lengths)
    }

    collatz_lengths.truncate(SIZE);

    for line in io::stdin().lock().lines() {
        let line_str = line.unwrap();
        let mut split = line_str.split_whitespace();
        let a: usize = split.next().unwrap().parse().unwrap();
        let b: usize = split.next().unwrap().parse().unwrap();

        let mut m: usize = 0;
        for i in a..b {
            if collatz_lengths[i] > m {
                m = collatz_lengths[i];
            }
        }
        println!("{a} {b} {m}")
    }
}
