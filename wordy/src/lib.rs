pub struct WordProblem {
    command: String,
}

impl WordProblem {
    pub fn new(command: &str) -> WordProblem {
        WordProblem { command: command.to_string() }
    }

    pub fn answer(&self) -> Result<i64, ()> {
        if !self.command.starts_with("What is ") {
            return Err(());
        }

        let command = self.command
            .replace("multiplied by", "multiplied_by")
            .replace("divided by", "divided_by");

        let mut parser = Parser::new(command.as_str());

        let mut ans = parser.next_num()?;

        while parser.has_next() {

            let op = parser.next_op()?;

            match op {
                "*" => {
                    let n = parser.next_num()?;
                    ans *= n;
                }
                "/" => {
                    let n = parser.next_num()?;
                    ans /= n;
                }
                "+" => {
                    let n = parser.next_num()?;
                    ans += n;
                }
                "-" => {
                    let n = parser.next_num()?;
                    ans -= n;
                }
                _ => return Err(()),
            }
        }

        Ok(ans)
    }
}

struct Parser {
    words: Vec<String>,
    pos: usize,
}

impl Parser {
    pub fn new(s: &str) -> Parser {
        let words: Vec<String> = s.split_terminator(
            |c: char| c.is_whitespace() || c == '?',
        ).skip(2)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        Parser {
            words: words,
            pos: 0,
        }
    }

    pub fn has_next(&self) -> bool {
        self.pos < self.words.len()
    }

    pub fn next_op(&mut self) -> Result<&'static str, ()> {
        let mut iter = self.words[self.pos..].iter().map(|s| s.as_str());
        self.pos += 1;
        match iter.next() {
            Some("multiplied_by") => Ok("*"),
            Some("divided_by") => Ok("/"),
            Some("plus") => Ok("+"),
            Some("minus") => Ok("-"),
            _ => Err(()),
        }
    }

    pub fn next_num(&mut self) -> Result<i64, ()> {
        if self.pos >= self.words.len() {
            return Err(());
        }

        let n = self.words[self.pos].parse::<i64>().map_err(|_| ())?;
        self.pos += 1;
        Ok(n)
    }
}
