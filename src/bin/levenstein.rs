fn main() {
    let mut args = std::env::args().skip(1);
    let a = args.next().unwrap();
    let b = args.next().unwrap();
    let lev_dist = lev(&a, &b);
    println!("Levenstein distance between a={a} and b={b} is {lev_dist}");
}

fn lev(a: &str, b: &str) -> usize {
    if a.len() == 0 || b.len() == 0 {
        return a.len() + b.len();
    }

    let mut ac = a.chars();
    let mut bc = b.chars();

    let ha = ac.nth(0).unwrap();
    let hb = bc.nth(0).unwrap();

    let ta: &str = ac.as_str();
    let tb: &str = bc.as_str();

    if ha == hb {
        return lev(ta, tb);
    }

    return 1 + lev(a, tb).min(lev(ta, b)).min(lev(ta, tb));
}
