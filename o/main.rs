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

const MOD: usize = 1_000_000_000 + 7;

fn main() {
    input! {
        n: usize,
        input: [[usize; n]; n],
    }

    let n_set = 1usize << n;
    let res = construct_n0(1usize, 1..n_set, &|vec, s| {
        let the_man = s.count_ones() as usize - 1;
        input[the_man]
            .iter()
            .enumerate()
            .filter(|(i_woman, &b)| b == 1 && (s & (1usize << i_woman)) != 0)
            .map(|(i_woman, _)| vec[s ^ (1usize << i_woman)])
            .sum::<usize>()
            % MOD
    });

    println!("{}", res.last().unwrap());
}
