#[cfg(test)]
mod tests {
    use ::*;

    const EXAMPLE: &str = "0    2\n7 0";
    #[test]
    fn it_works() {
        assert_eq!(find_loop(EXAMPLE), (5, 4));
    }
}


pub fn find_loop(oh_boy_oh_boy: &str) -> (i32, i32) {
    use std::str::FromStr;
    let mut seen: Vec<Vec<i32>> = vec![
        oh_boy_oh_boy
            .split_whitespace()
            .map(|s_num| i32::from_str(s_num).unwrap())
            .collect(),
    ];
    let rep: Vec<i32>;
    let mut i = 0;
    // because loop... geddit?
    loop {
        // <--- found one!
        let current = &seen[seen.len() - 1].clone();
        let mut biggest = *current.iter().max().unwrap();
        let pos = current.iter().position(|&n| n == biggest).unwrap();
        let mut next = current.clone();
        next[pos] = 0;
        let mut runner = pos + 1;
        loop {
            // <--- found another! I'm so good at this
            if runner >= next.len() {
                runner = 0;
            }
            next[runner] += 1;
            biggest -= 1;
            runner += 1;
            if biggest <= 0 {
                break;
            }
        }
        i += 1;
        if seen.contains(&next) {
            rep = next;
            break;
        }
        seen.push(next);
    }
    let pos = seen.iter().position(|n| n == &rep).unwrap();
    let distance = seen.len() - pos;
    (i, distance as i32)
}
