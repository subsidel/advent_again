#[cfg(test)]
mod tests {
    use is_pass_non_repeaty;
    use count_valid;
    const NO_REPEATES_1: &str = "aa bb cc dd ee";
    const NO_REPEATES_2: &str = "aa bb cc dd aaa";
    const REPEATS_1: &str = "aa bb cc dd aa";

    #[test]
    fn can_find_non_repeaty_passes() {
        assert_eq!(is_pass_non_repeaty(NO_REPEATES_1), true);
        assert_eq!(is_pass_non_repeaty(NO_REPEATES_2), true);
        assert_eq!(is_pass_non_repeaty(REPEATS_1), false);
    }

    #[test]
    fn can_count_non_repeaty_passes() {
        let no_reps = |s: &str| is_pass_non_repeaty(s);
        assert_eq!(
            count_valid(
                &format!("{}\n{}\n{}", NO_REPEATES_1, NO_REPEATES_2, REPEATS_1),
                &no_reps
            ),
            2
        );
        assert_eq!(
            count_valid(
                &format!(
                    "{}\n{}\n{}\n{}",
                    NO_REPEATES_1,
                    NO_REPEATES_2,
                    REPEATS_1,
                    NO_REPEATES_1
                ),
                &no_reps
            ),
            3
        );
    }

    use contains_no_anagrams;
    const NO_ANAS_1: &str = "abcde fghij";
    const NO_ANAS_2: &str = "a ab abc abd abf abj";
    const NO_ANAS_3: &str = "iiii oiii ooii oooi oooo";
    const ANA_1: &str = "abcde xyz ecdab";
    const ANA_2: &str = "oiii ioii iioi iiio";

    #[test]
    fn can_find_anagrams() {
        assert_eq!(contains_no_anagrams(NO_ANAS_1), true);
        assert_eq!(contains_no_anagrams(NO_ANAS_2), true);
        assert_eq!(contains_no_anagrams(NO_ANAS_3), true);
        assert_eq!(contains_no_anagrams(ANA_1), false);
        assert_eq!(contains_no_anagrams(ANA_2), false);
    }

    #[test]
    fn can_count_non_anagrammy_passes() {
        let no_anas = |s: &str| contains_no_anagrams(s);
        assert_eq!(
            count_valid(
                &format!(
                    "{}\n{}\n{}\n{}\n{}",
                    NO_ANAS_1,
                    NO_ANAS_2,
                    NO_ANAS_3,
                    ANA_1,
                    ANA_2
                ),
                &no_anas
            ),
            3
        );
        assert_eq!(
            count_valid(
                &format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                    NO_ANAS_1,
                    NO_ANAS_2,
                    NO_ANAS_3,
                    ANA_1,
                    ANA_2,
                    NO_ANAS_1,
                    NO_ANAS_2,
                    NO_ANAS_3,
                    ANA_1,
                    ANA_2
                ),
                &no_anas
            ),
            6
        );
    }
}

pub fn contains_no_anagrams(pass: &str) -> bool {
    use std::iter::FromIterator;
    // normalise each "word"
    // check if non-repeaty
    let normalised: String = pass.split(' ')
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            String::from_iter(chars)
        })
        .collect::<Vec<String>>()
        .join(" ");
    is_pass_non_repeaty(&normalised)
}

pub fn count_valid(pass_list: &str, validator: &Fn(&str) -> bool) -> usize {
    pass_list
        .split('\n')
        .filter(|p| validator(p))
        .collect::<Vec<&str>>()
        .len()
}

pub fn is_pass_non_repeaty(pass: &str) -> bool {
    pass.split(' ')
        .fold((true, vec![""]), |(has_repeat, mut words), next_word| {
            let has_repeat = has_repeat && !words.contains(&next_word);
            words.extend(vec![next_word]);
            (has_repeat, words)
        })
        .0
}
