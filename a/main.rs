use proconio::input;

fn construct_n0<I, T>(
    x0: T,
    items: impl IntoIterator<Item = I, IntoIter = impl ExactSizeIterator<Item = I>>,
    f: &impl Fn(I, &[T]) -> T,
) -> Vec<T> {
    let items = items.into_iter();

    let mut dp = Vec::with_capacity(items.len() + 1);
    dp.push(x0);

    for item in items {
        dp.push(f(item, &dp));
    }

    dp
}

fn main() {
    input! {
        n: usize,
        hs: [isize; n],
    }

    let res = construct_n0(0, &hs[1..], &|&h1, vec| {
        vec.iter()
            .zip(&hs)
            .rev()
            .take(2)
            .map(|(acc, h2)| acc + (h2 - h1).abs())
            .min()
            .unwrap_or(h1)
    });

    println!("{}", res.last().unwrap());
}
