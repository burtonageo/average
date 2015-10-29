/*
 * Written by George Burton on the 15th Jan, 2014
 *
 * average - a warmup exercise for PS1 from CSC4414 from University of Virginia
 * file: main.rs
 *
 * This is free and unencumbered software released into the public domain.   
 */

#![crate_id = "average"]

use std::os;

mod mean;

/// Simple CLI app to calculate the mean of numerical arguments passed to
/// it via the command line.
fn main() {
    let args = os::args();
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
fn parse_args<'a, 'b>(args: &'a Vec<&str>) -> (&'a Vec<u32>, &'a Vec<&'b str>) {
    // We go to the trouble of storing the bad arguments rather than discarding them
    // (and printing out 'Bad Input') so that unit testing this function does not
    // clutter our console with unnecessary messages, and so we know we're getting
    // the bad data (rather than throwing it to the wind and thus printing nothing).
    // Slightly inefficient, but convenient.

    let mut nums: Vec<u32>;
    let mut bad_args: Vec<&str>;

    for arg in args.iter().skip(1) {
        let n_opt: Option<u32> = from_str(*arg);
        match n_opt {
            Some(n) => nums.push(n),
            None    => bad_args.push(arg.to_owned())
        };
    }
    return (nums, bad_args);
}

#[cfg(test)]
mod test {
    static PROGM_NAME: &'static str = "average";

    #[test]
    fn test_parse_args_empty_vec() {
        let test_vec = [PROGM_NAME.to_owned()];
        assert_eq!(super::parse_args(test_vec), ([], []));
    }

    #[test]
    fn test_parse_args_vec_with_numbers() {
        let test_vec = [PROGM_NAME.to_owned(), "1", "2", "3"];
        assert_eq!(super::parse_args(test_vec), ([1, 2, 3], []));
    }

    #[test]
    fn test_parse_args_vec_with_all_strings() {
        let test_vec = [PROGM_NAME.to_owned(), "Hello", "Judy", "How", "Are", "You"];
        assert_eq!(
            super::parse_args(test_vec),
            ([], ["Hello", "Judy", "How", "Are", "You"]));
    }

    #[test]
    fn test_parse_args_vec_with_mixed() {
        let test_vec = [PROGM_NAME.to_owned(), "Pears: ", "1", "Apples: ", "23"];
        assert_eq!(
            super::parse_args(test_vec),
            ([1, 23], ["Pears: ", "Apples: "]));
    }
}
