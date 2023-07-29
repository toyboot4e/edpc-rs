use proconio::input;

fn construct_n0<I, T>(
    x0: T,
    items: impl IntoIterator<Item = I, IntoIter = impl ExactSizeIterator<Item = I>>,
    f: &impl Fn(&[T], I) -> T,
) -> Vec<T> {
    let items = items.into_iter();

    let mut dp = Vec::with_capacity(items.len() + 1);
    dp.push(x0);

    for item in items {
        dp.push(f(&dp, item));
    }

    dp
}

fn main() {
    input! {
        n: usize,
        hs: [isize; n],
    }

    let res = construct_n0(0, &hs[1..], &|vec, &h1| {
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
