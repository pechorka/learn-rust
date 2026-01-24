use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Strategy {
    Naive,
    Idiomatic,
    Matrix,
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "naive" => Ok(Self::Naive),
            "idiomatic" | "ideomatic" => Ok(Self::Idiomatic),
            "matrix" => Ok(Self::Matrix),
            _ => Err(format!("unknown strategy {s}")),
        }
    }
}

pub fn levenshtein(a: &str, b: &str, strategy: Strategy) -> usize {
    match strategy {
        Strategy::Naive => lev_naive(a, b),
        Strategy::Idiomatic => lev_idiomatic(a, b),
        Strategy::Matrix => lev_matrix(a, b),
    }
}

pub fn lev_naive(a: &str, b: &str) -> usize {
    if a.is_empty() {
        return b.chars().count();
    }
    if b.is_empty() {
        return a.chars().count();
    }

    let mut ac = a.chars();
    let mut bc = b.chars();

    let ha = ac.next().unwrap();
    let hb = bc.next().unwrap();

    let ta = ac.as_str();
    let tb = bc.as_str();

    if ha == hb {
        return lev_naive(ta, tb);
    }

    1 + lev_naive(ta, b)
        .min(lev_naive(a, tb))
        .min(lev_naive(ta, tb))
}

pub fn lev_idiomatic(a: &str, b: &str) -> usize {
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
                lev_idiomatic(ta, tb)
            } else {
                1 + lev_idiomatic(a, tb)
                    .min(lev_idiomatic(ta, b))
                    .min(lev_idiomatic(ta, tb))
            }
        }
    }
}

pub fn lev_matrix(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    let mut m: Vec<Vec<usize>> = vec![vec![0; b_len + 1]; a_len + 1];

    for i in 1..=a_len {
        m[i][0] = i;
    }

    for j in 1..=b_len {
        m[0][j] = j;
    }

    for (i, ac) in a.chars().enumerate() {
        for (j, bc) in b.chars().enumerate() {
            let substitution_cost = usize::from(ac != bc);
            let mi = i + 1;
            let mj = j + 1;

            m[mi][mj] = (m[mi - 1][mj] + 1) // deletion
                .min(m[mi][mj - 1] + 1) // insertion
                .min(m[mi - 1][mj - 1] + substitution_cost); // substitution
        }
    }

    m[a_len][b_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lev_naive_matches_known_values() {
        test_impl(lev_naive);
    }

    #[test]
    fn lev_idiomatic_matches_known_values() {
        test_impl(lev_idiomatic);
    }

    #[test]
    fn lev_matrix_matches_known_values() {
        test_impl(lev_matrix);
    }

    fn test_impl(impl_fn: fn(&str, &str) -> usize) {
        let cases = [
            ("", "", 0),
            ("", "abc", 3),
            ("abc", "", 3),
            ("kitten", "sitting", 3),
            ("gumbo", "gambol", 2),
            ("str1", "str2", 1),
        ];

        for (a, b, expected) in cases {
            assert_eq!(impl_fn(a, b), expected, "a={a}, b={b}");
        }
    }
}
