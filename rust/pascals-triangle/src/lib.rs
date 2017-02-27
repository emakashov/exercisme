pub struct PascalsTriangle {
    n: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { n: row_count}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut triangle: Vec<Vec<u32>> = Vec::new();

        for i in 0..self.n as usize {
            let mut row: Vec<u32> = vec![1; i as usize + 1];
            if i > 1 {
                for j in 1..i as usize {
                    row[j] = triangle[i - 1][j - 1] + triangle[i - 1][j]
                }
            }
            triangle.push(row)
        }

        triangle
    }
}
