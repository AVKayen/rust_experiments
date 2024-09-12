// calculate the lengths of shortened ((3n+1)/2 instead of (3n+1)) paths of Collatz conjecture
// between 2 and SIZE, giving a collatz_lenghts vector

const SIZE: usize = 10001;

fn calculate_collatz_length(n: usize, collatz_lengths: &mut Vec<usize>) {
    let mut x = n;
    let mut current_seq: Vec<usize> = vec![];

    while collatz_lengths[x] == 0 {
        current_seq.push(x);
        if x % 2 == 0 {
            x /= 2;
        } else {
            x = (3 * x + 1) / 2;
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

    println!("{}", collatz_lengths[2137])
}
