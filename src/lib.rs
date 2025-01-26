//! This is a silly example repo.
//! It's only purpose is to demonstrate usage of the following command:
//! `cargo +nightly test -- --list -Zunstable-options --format=json`

mod no_testing;

pub use no_testing::not_tested;

/// Simple add function
///
/// # Example
/// ```
/// use adder::add;
///
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Adds 2 to the input value
///
/// # Example
/// ```
/// use adder::add_two;
///
/// let result = add_two(2);
/// assert_eq!(result, 4);
/// ```
pub fn add_two(value: u64) -> u64 {
    value + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
