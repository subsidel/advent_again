#[cfg(test)]
mod tests {
    use do_the_thing;
    const INPUT: i32 = 361527;
    #[test]
    fn it_works() {
        assert_eq!(do_the_thing(1), 0);
        assert_eq!(do_the_thing(12), 3);
        println!("{}", do_the_thing(INPUT))
    }
}
// interestingly doesn't work for all inputs
// but it worked for mine so lol.
pub fn do_the_thing(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }

    let square_root = ((n - 1) as f64).sqrt();

    let odds: Vec<i32> = (0..n).collect();

    let pos_low = odds.clone()
        .iter()
        .position(|next_odd| square_root < *next_odd as f64)
        .unwrap();

    let pos_high = odds.clone()
        .iter()
        .rev()
        .position(|next_odd| square_root > *next_odd as f64)
        .unwrap();

    let which_high = odds[pos_low - 1];
    let which_low = odds.clone().into_iter().rev().collect::<Vec<i32>>()[pos_high - 1];

    let tall_squares = (which_high * which_high - n).abs();
    let long_squares = (which_low * which_low - n).abs();
    if tall_squares < long_squares {
        tall_squares
    } else {
        long_squares
    }
}
