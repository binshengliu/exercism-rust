use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input
        .iter()
        .flat_map(|(&i, v)| {
            v.iter().map(
                move |c| (c.to_lowercase().next().unwrap(), i),
            )
        })
        .collect()
}
