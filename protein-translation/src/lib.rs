use std::collections::HashMap;

const STOP: &str = &"stop codon";

pub struct Info {
    pairs: HashMap<&'static str, &'static str>,
}

impl Info {
    pub fn name_for(&self, codon: &str) -> Result<&'static str, &'static str> {
        self.pairs.get(codon).cloned().ok_or("")
    }

    pub fn of_rna(&self, seq: &str) -> Result<Vec<&'static str>, &'static str> {
        seq.chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|chars| chars.iter().collect::<String>())
            .take_while(|codon| self.pairs.get(&codon.as_str()) != Some(&STOP))
            .map(|codon| self.pairs.get(&codon.as_str()).cloned().ok_or(""))
            .collect()
    }
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Info {
    Info { pairs: pairs.into_iter().collect() }
}
