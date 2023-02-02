#[path = "utils.rs"]
mod utils;

pub struct Puzzle {
    pub input: Vec<String>,
}
impl Puzzle {
    /// Returns the solution for the second problem
    pub fn first(&self) -> String {
        let mut max = 0;
        let mut sum = 0;
        for line in &self.input {
            match line.parse::<i32>() {
                Ok(num) => sum += num,
                Err(_) => {
                    if max < sum {
                        max = sum;
                    }
                    sum = 0;
                }
            }
        }
        return max.to_string();
    }

    /// Returns the solution for the second problem
    pub fn second(&self) -> String {
        let mut sum = 0;
        let mut calorie_count: Vec<i32> = Vec::new();
        for line in &self.input {
            match line.parse::<i32>() {
                Ok(num) => sum += num,
                Err(_) => {
                    calorie_count.push(sum);
                    sum = 0;
                }
            }
        }
        calorie_count.sort_by(|a, b| b.cmp(a));
        let top_3: i32 = (&calorie_count[0..3]).iter().sum();
        return top_3.to_string();
    }
}
