///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
#[allow(unused_variables)]
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    if from_base <= 1 || to_base <= 1{
        return Err(());
    }

    // The "?" avoids explicitly using a if or match to check for any
    // error.
    let mut val: u32 = number.
        iter().
        rev().
        enumerate().
        map(|(index, num)| {
            if num < &from_base {
                Ok(num * from_base.pow(index as u32))
            } else {
                Err(())
            }
        }).collect::<Result<Vec<u32>, ()>>()?.iter().sum();

    let mut ans = Vec::new();
    while val != 0 {
        ans.insert(0, val % to_base);
        val = val / to_base;
    }
    Ok(ans)
}

// See
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
// for the use of "collect" over a list of Result<>.

// See
// https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-
// for the use of "?" as a shortcut for propagating errors.
