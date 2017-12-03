#[cfg(test)]
mod tests {
    use do_the_thing;
    use do_the_thing_gold;
    const NOT_GOLD_TEST: &str = "5 1 9 5
7 5 3
2 4 6 8";
    const GOLD_TEST: &str = "5 9 2 8
9 4 7 3
3 8 6 5";

    #[test]
    fn not_gold() {
        assert_eq!(do_the_thing(NOT_GOLD_TEST), 18);
    }
    #[test]
    fn gold() {
        assert_eq!(do_the_thing_gold(GOLD_TEST), 9);
    }
}

pub fn do_the_thing(s: &str) -> i32 {
    use std::cmp;

    s.split('\n')
        .map(|row| {
            let (biggy, smalls) =
                row.split_whitespace()
                    .fold((0, std::i32::MAX), |(biggy, smalls), stringy_num| {
                        let nummy_num = stringy_num.parse::<i32>().unwrap();
                        (cmp::max(biggy, nummy_num), cmp::min(smalls, nummy_num))
                    });
            biggy - smalls
        })
        .sum()
}

extern crate itertools;
pub fn do_the_thing_gold(s: &str) -> i32 {
    use itertools::Itertools;
    let lineos: Vec<Vec<i32>> = s.split('\n')
        .map(|row| {
            row.split_whitespace()
                .filter_map(|stringy_num| stringy_num.parse::<i32>().ok())
                .collect()
        })
        .collect();

    lineos
        .iter()
        .filter_map(|els| {
            els.iter()
                .tuple_combinations()
                .filter_map(|(a, b)| {
                    // https://i.redd.it/vapihas0w0vz.png
                    if a % b == 0 {
                        Some(a / b)
                    } else if b % a == 0 {
                        Some(b / a)
                    } else {
                        None
                    }
                })
                .next()
        })
        .sum()
}
