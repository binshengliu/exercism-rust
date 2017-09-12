use std::collections::HashMap;

#[derive(Debug)]
pub struct RibonucleicAcid {
    contents: String,
}

impl RibonucleicAcid {
    pub fn new(s: &str) -> Self {
        RibonucleicAcid { contents: s.to_string() }
    }
}

impl PartialEq for RibonucleicAcid {
    fn eq(&self, other: &RibonucleicAcid) -> bool {
        self.contents == other.contents
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

    pub fn to_rna(&self) -> Result<RibonucleicAcid, String> {
        if self.contents.chars().any(|c| !self.map.contains_key(&c)) {
            Err(String::from("Unknown symbol found"))
        } else {
            Ok(RibonucleicAcid::new(
                self.contents
                    .chars()
                    .map(|c| self.map[&c])
                    .collect::<String>()
                    .as_str(),
            ))
        }
    }
}
