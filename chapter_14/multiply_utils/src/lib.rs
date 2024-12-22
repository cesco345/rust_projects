/// Multiplies the input by two.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = multiply_utils::multiply_by_two(arg);
///
/// assert_eq!(10, answer);
/// ```
pub fn multiply_by_two(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(8, multiply_by_two(4));
    }
}
