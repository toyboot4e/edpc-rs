use proconio::input;
use std::iter;

fn main() {
    input! {
        (n, w): (usize, usize),
        input: [(usize, usize); n],
    }

    let res = input.iter().fold(
        iter::once(Some(0))
            .chain(iter::repeat(None).take(w))
            .collect::<Vec<_>>(),
        |vec, (dw, dv)| {
            eprintln!("{:?}", vec);
            vec.iter()
                .enumerate()
                .map(|(i, &acc)| {
                    let v1 = vec
                        // FIXME: This is wrong
                        .get((i as isize - *dw as isize) as usize)
                        .cloned()
                        .flatten()
                        .map(|v1| v1 + *dv);
                    [v1, acc].iter().flatten().cloned().max()
                })
                .collect::<Vec<_>>()
        },
    );

    println!("{}", res.iter().flatten().cloned().max().unwrap());
}
