// See
// https://github.com/rust-lang/rust/issues/24140#issuecomment-90626264
// for difference between Borrow and AsRef.
//
// Borrow only converts a T to &T. For example, vec![1, 2] can only be
// converted to &Vec, not to &[1, 2].
//
// AsRef can convert multiple types to &T. For example, vec![1, 2],
// [1, 2], and &[1, 2], can both be converted to &[1, 2].
//
// Also see the signature of `fn open<P: AsRef<Path>>(path: P) ->
// Result<File>`. Any type that can be converted to &Path is accepted
// by the function.

pub fn find<T, A>(list: A, value: T) -> Option<usize>
where
    T: PartialOrd,
    A: AsRef<[T]>,
{
    let list = list.as_ref();
    if list.is_empty() {
        return None;
    }

    let mut begin = 0;
    let mut end = list.len() - 1;

    while begin <= end {
        let middle = (begin + end) / 2;
        if value < list[middle] {
            end = match middle.checked_sub(1) {
                Some(v) => v,
                None => return None,
            };
        } else if value > list[middle] {
            begin = middle + 1;
        } else {
            return Some(middle);
        }
    }
    None
}
