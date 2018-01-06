#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => (
        {
            let mut map = HashMap::new();
            $(map.insert($k, $v);)*
            map
        }
    );
    ($($k:expr => $v:expr,)*) => (
        hashmap![$($k => $v),*]
    )
}
