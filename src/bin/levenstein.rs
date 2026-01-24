use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug)]
enum Strategy {
    Naive,
    Ideomatic,
    Matrix,
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "naive" => Ok(Self::Naive),
            "ideomatic" => Ok(Self::Ideomatic),
            "matrix" => Ok(Self::Matrix),
            _ => Err(format!("unknown strategy {s}")),
        }
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let a = args.next().expect("no string a");
    let b = args.next().expect("no string b");

    let strategy = if let Some(strat) = args.next() {
        Strategy::from_str(&strat).expect("invalid strategy provided")
    } else {
        Strategy::Naive
    };

    let lev_dist = match strategy {
        Strategy::Naive => lev(&a, &b),
        Strategy::Ideomatic => lev_ideomatic(&a, &b),
        Strategy::Matrix => lev_matrix(&a, &b),
    };

    println!(
        "Levenstein distance between a={a} and b={b} using strategy {:#?} is {lev_dist}",
        strategy
    );
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

    1 + lev(ta, b).min(lev(a, tb)).min(lev(ta, tb))
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

fn lev_matrix(a: &str, b: &str) -> usize {
    let mut m: Vec<Vec<usize>> = vec![vec![0; b.len() + 1]; a.len() + 1];

    // a can be reduced to empty string by droppin i chars
    for i in 1..=a.len() {
        m[i][0] = i;
    }

    // b can be buit from empty string by adding j chars
    for j in 1..=b.len() {
        m[0][j] = j;
    }

    for (j, bc) in b.chars().enumerate() {
        for (i, ac) in a.chars().enumerate() {
            let substitution_cost = if ac == bc { 0 } else { 1 };
            let mi = i + 1;
            let mj = j + 1;

            m[mi][mj] = (m[mi - 1][mj] + 1) // deletion
                .min(m[mi][mj - 1] + 1) // insertion
                .min(m[mi - 1][mj - 1] + substitution_cost); // substitution
        }
    }
    for row in &m {
        println!("{:?}", row);
    }

    m[a.len()][b.len()]
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

    #[test]
    fn test_lev_matrix() {
        test_levenstein_impl(lev_matrix);
    }

    fn test_levenstein_impl(impl_fn: fn(&str, &str) -> usize) {
        let a = "str1";
        assert_eq!(impl_fn(a, a), 0);

        let a = "str1";
        let b = "str2";
        assert_eq!(impl_fn(a, b), 1);
    }
}
