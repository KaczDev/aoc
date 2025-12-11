use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// The output is wrapped in a Result to allow matching on errors.
/// Returns an Iterator to the Reader of the lines of the file.
///
/// Example usage:
/// ```
/// fn read_input(file_name: &str) -> Result<Input<X>> {
///     Ok(aoc_utils::reader::read_lines(file_name)?
///         .map_while(Result::ok)
///         .map(|line| {
///             line.parse()
///                 .expect(&format!("Couldn't parse line - '{}'", line))
///         })
///         .collect())
/// }
/// ```
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
