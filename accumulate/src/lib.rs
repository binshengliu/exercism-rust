pub fn map_function<I: Copy, T: Fn(I) -> I>(input: Vec<I>, func: T) -> Vec<I> {
    input.iter().map(|x| func(*x)).collect()
}

// Either implementation is ok. The difference is that input is
// invalid after into_iter.
pub fn map_closure<I, T: Fn(I) -> I>(input: Vec<I>, func: T) -> Vec<I> {
    input.into_iter().map(|x| func(x)).collect()
}
