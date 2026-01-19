fn main() {
    let mut args = std::env::args().skip(1);
    let a = args.next().unwrap();
    let b = args.next().unwrap();
    let lev_dist = lev(&a, &b);
    println!("Levenstein distance between a={a} and b={b} is {lev_dist}");
}

fn lev(a: &str, b: &str) -> usize {
    let mut ac = a.chars();
    let mut bc = b.chars();
    if a.is_empty() || b.is_empty() {
        return ac.count() + bc.count();
    }

    let ha = ac.next().unwrap();
    let hb = bc.next().unwrap();

    let ta = ac.as_str();
    let tb = bc.as_str();

    if ha == hb {
        return lev(ta, tb);
    }

    1 + lev(a, tb).min(lev(ta, b)).min(lev(ta, tb))
}
