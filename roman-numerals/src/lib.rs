use std::fmt;

const MAP: [(u16, &str); 13] = [
    (1, "I"),
    (4, "IV"),
    (5, "V"),
    (9, "IX"),
    (10, "X"),
    (40, "XL"),
    (50, "L"),
    (90, "XC"),
    (100, "C"),
    (400, "CD"),
    (500, "D"),
    (900, "CM"),
    (1_000, "M"),
];

pub struct Roman {
    string: String,
}

impl Roman {
    pub fn from(num: u16) -> Roman {
        Roman { string: Roman::num_to_string(num) }
    }

    fn num_to_string(num: u16) -> String {
        let mut n = num;
        MAP.iter().rev().fold(String::new(), |mut string,
         &(value, symbol)| {
            while n >= value {
                string += &symbol.to_string();
                n -= value;
            }
            string
        })
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}
