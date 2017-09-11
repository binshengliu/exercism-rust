pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { rows: Self::create(row_count) }
    }

    fn create(row_count: u32) -> Vec<Vec<u32>> {
        if row_count == 0 {
            return Vec::new();
        }

        let mut result: Vec<Vec<u32>> = vec![vec![1]];
        for _ in 1..row_count {
            let prev_row = result.last().unwrap().clone();
            let mut new_row: Vec<u32> = vec![1];
            let mut middle_part = prev_row
                .iter()
                .zip(prev_row.iter().skip(1))
                .map(|(&a, &b)| a + b)
                .collect::<Vec<u32>>();
            new_row.append(&mut middle_part);
            new_row.push(1);

            result.push(new_row.clone());
        }

        result
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
