use std::iter::FromIterator;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    contents: String,
}

impl RibonucleicAcid {
    pub fn new(s: &str) -> Self {
        RibonucleicAcid { contents: s.to_string() }
    }
}

impl FromIterator<char> for RibonucleicAcid {
    fn from_iter<I: IntoIterator<Item=char>>(iter: I) -> Self {
        RibonucleicAcid {contents: iter.into_iter().collect()}
    }
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
    contents: String,
    map: HashMap<char, char>,
}

impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> Self {
        let map: HashMap<char, char> =
            [('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]
                .iter()
                .map(|&(a, b)| (a, b))
                .collect();

        DeoxyribonucleicAcid {
            contents: s.to_string(),
            map: map,
        }
    }

    // See
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
    // and
    // http://exercism.io/submissions/05f72a0b6c2f44199eeca556b0800b3b
    // for the idea of collecting Result<Item, _> into
    // Result<Collection, _>
    pub fn to_rna(&self) -> Result<RibonucleicAcid, String> {
        self.contents
            .chars()
            .map(|c| if self.map.contains_key(&c) {
                Ok(self.map[&c])
            } else {
                Err(format!("Unknown symbol {}", c))
            })
            .collect()
    }
}
