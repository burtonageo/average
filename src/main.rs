/*
 * Written by George Burton on the 15th Jan, 2014
 *
 * average - a warmup exercise for PS1 from CSC4414 from University of Virginia
 * file: main.rs
 *
 * This is free and unencumbered software released into the public domain.
 */

use std::env::args;

mod mean;

/// Simple CLI app to calculate the mean of numerical arguments passed to
/// it via the command line.
fn main() {
    let args = args().collect::<Vec<_>>();
    let (nums, bad_args) = parse_args(args);

    for bad_arg in bad_args.iter() {
        println!("Bad Input: {}", *bad_arg);
    }

    match mean::mean(nums) {
        Some(a) => println!("{}", a),
        None    => println!("No numbers given.")
    };
}

/// Parses the argument vector to look for uints
///
/// Returns a tuple of two lists; the first contains all of the numbers found
/// in the parameter vector, and the second contains all of the strings which
/// could not be parsed.
fn parse_args(args: Vec<String>) -> (Vec<u32>, Vec<String>) {
    // We go to the trouble of storing the bad arguments rather than discarding them
    // (and printing out 'Bad Input') so that unit testing this function does not
    // clutter our console with unnecessary messages, and so we know we're getting
    // the bad data (rather than throwing it to the wind and thus printing nothing).
    // Slightly inefficient, but convenient.

    let mut nums = vec![];
    let mut bad_args = vec![];

    for arg in args.into_iter().skip(1) {
        use std::str::FromStr;
        let n_opt = FromStr::from_str(&arg[..]);
        match n_opt {
            Ok(n)  => nums.push(n),
            Err(_) => bad_args.push(arg)
        };
    }
    return (nums, bad_args);
}

#[cfg(test)]
mod test {
    static PROGM_NAME: &'static str = "average";

    #[test]
    fn test_parse_args_empty_vec() {
        let test_vec = vec![PROGM_NAME.into()];
        assert_eq!(super::parse_args(test_vec), (vec![], vec![]));
    }

    #[test]
    fn test_parse_args_vec_with_numbers() {
        let test_vec = vec![PROGM_NAME.into(), "1".into(), "2".into(), "3".into()];
        assert_eq!(super::parse_args(test_vec), (vec![1, 2, 3], vec![]));
    }

    #[test]
    fn test_parse_args_vec_with_all_strings() {
        let test_vec = vec![PROGM_NAME.into(), "Hello".into(), "Judy".into(), "How".into(), "Are".into(), "You".into()];
        assert_eq!(
            super::parse_args(test_vec),
            (vec![], vec!["Hello".into(), "Judy".into(), "How".into(), "Are".into(), "You".into()]));
    }

    #[test]
    fn test_parse_args_vec_with_mixed() {
        let test_vec = vec![PROGM_NAME.into(), "Pears: ".into(), "1".into(), "Apples: ".into(), "23".into()];
        assert_eq!(
            super::parse_args(test_vec),
            (vec![1, 23], vec!["Pears: ".into(), "Apples: ".into()]));
    }
}
