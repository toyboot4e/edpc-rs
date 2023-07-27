use proconio::input;

fn main() {
    input! {
        n: usize,
        input: [(u32, u32, u32); n],
    }

    let res = input
        .iter()
        .fold([0, 0, 0], |[acc1, acc2, acc3], (x1, x2, x3)| {
            [
                u32::max(acc2, acc3) + x1,
                u32::max(acc3, acc1) + x2,
                u32::max(acc1, acc2) + x3,
            ]
        });

    let res = res.iter().max().unwrap();
    println!("{}", res);
}
