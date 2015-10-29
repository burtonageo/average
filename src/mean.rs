/*
 * Written by George Burton on the 15th Jan, 2014
 *
 * average - a warmup exercise for PS1 from CSC4414 from University of Virginia
 * file: mean.rs
 *
 * This is free and unencumbered software released into the public domain.
 */

/// Calculates the mean of a vector of uints as a float
///
/// Returns an Option which contains a float. If the list is empty,
/// then the Option is None, otherwise it contains the calculated mean.
pub fn mean(numbers: Vec<u32>) -> Option<f64> {
    if numbers.len() == 0 {
        return None;
    }

    let mut average = 0f64;
    let mut count = 0u32;
    for n in numbers.iter() {
        average += *n as f64;
        count += 1;
    }
    Some((average as f64)/(count as f64))
}

#[cfg(test)]
mod test {
    use super::mean;

    #[test]
    fn average_test_good_input_1() {
        let test_vector = [1, 2, 3, 4, 5];
        assert_eq!(mean(test_vector), Some(3.0));
    }

    #[test]
    fn average_test_good_input_2() {
        let test_vector = [12, 23, 34, 45, 56];
        assert_eq!(mean(test_vector), Some(34.0));
    }

    #[test]
    fn average_test_empty_input() {
        let test_vector = [];
        assert_eq!(mean(test_vector), None);
    }
}
