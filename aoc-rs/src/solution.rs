use anyhow::Result;


pub trait Solution {
    fn part_a(&self, file_name: &str) -> Result<String>;
    fn part_b(&self, file_name: &str) -> Result<String>;
}
