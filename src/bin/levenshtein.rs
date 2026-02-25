use learn_rust::algorithms::levenshtein::{Strategy, levenshtein};

fn main() {
    let mut args = std::env::args().skip(1);
    let a = args.next().expect("no string a");
    let b = args.next().expect("no string b");

    let strategy = args
        .next()
        .as_deref()
        .map(|s| s.parse().expect("invalid strategy provided"))
        .unwrap_or(Strategy::Naive);

    let lev_dist = levenshtein(&a, &b, strategy);

    println!(
        "Levenshtein distance between a={a} and b={b} using strategy {:?} is {lev_dist}",
        strategy,
    );
}
