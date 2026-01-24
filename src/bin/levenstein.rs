fn main() {
    let mut args = std::env::args().skip(1);
    let a = args.next().expect("no string a");
    let b = args.next().expect("no string b");
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

fn lev_ideomatic(a: &str, b: &str) -> usize {
    let mut ac = a.chars();
    let mut bc = b.chars();

    match (ac.next(), bc.next()) {
        (None, None) => 0,
        (Some(_), None) => ac.count() + 1,
        (None, Some(_)) => bc.count() + 1,
        (Some(ha), Some(hb)) => {
            let ta = ac.as_str();
            let tb = bc.as_str();

            if ha == hb {
                lev(ta, tb)
            } else {
                1 + lev(a, tb).min(lev(ta, b)).min(lev(ta, tb))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lev() {
        test_levenstein_impl(lev);
    }

    #[test]
    fn test_lev_ideomatic() {
        test_levenstein_impl(lev_ideomatic);
    }

    fn test_levenstein_impl(impl_fn: fn(&str, &str) -> usize) {
        let a = "str1";
        assert_eq!(impl_fn(a, a), 0);

        let a = "str1";
        let b = "str2";
        assert_eq!(impl_fn(a, b), 1);
    }
}
