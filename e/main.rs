//! ```cargo
//! [dependencies]
//! proconio = "0.4.5"
//! ```

use proconio::input;
use std::iter;

fn main() {
    input! {
        (n, max_w): (usize, usize),
        input: [(usize, usize); n],
    }

    let cap = 100_000; // w * v

    let res = input.iter().fold(
        // value -> min weight
        iter::once(Some(0))
            .chain(iter::repeat(None).take(cap))
            .collect::<Vec<_>>(),
        |vec, (dw, dv)| {
            vec.iter()
                .enumerate()
                .map(|(i, &acc)| {
                    let w1 = vec
                        // FIXME: This is wrong
                        .get((i as isize - *dv as isize) as usize)
                        .cloned()
                        .flatten()
                        .map(|w1| w1 + *dw)
                        .filter(|&w| w <= max_w);
                    [w1, acc].iter().flatten().cloned().min()
                })
                .collect::<Vec<_>>()
        },
    );

    println!(
        "{}",
        res.iter()
            .enumerate()
            .filter_map(|(i, w)| w.map(|_| i))
            .max()
            .unwrap()
    );
}
