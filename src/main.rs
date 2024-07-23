use polars::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Set the input filename
    let filename = "/home/rhys/Documents/rew_test/test.dat";
    // Initialise variables so they can be set in the if statement
    let x_vals;
    let y_vals;
    // Call read_lines and unpack Result
    if let Ok(lines) = read_lines(filename) {
        // Each line is also a Result, so unpack those
        let lines = lines.map(|x| x.unwrap());
        // Split columns into f32 Vec 
        (x_vals, y_vals) = read_xy_pairs(lines);
    } else {
        // If something didn't work, panic!
        panic!()
    }
    // Check that the variables were read correctly
    println!("{:?}", x_vals);
    
    // Create a dataframe from the two Vecs with the data
    let df: PolarsResult<DataFrame> = df!(
        "X" => x_vals,
        "Y" => y_vals,
    );
    
    // DataFrame is ALSO a Result, so unpack THAT.
    let df = df.unwrap();
    
    // Print head to ensure all is well.
    println!("{}", df.head(Some(3)));
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_xy_pairs(i: impl Iterator<Item = String>) -> (Vec<f32>, Vec<f32>) {
    i.map(|string| -> Vec<f32> {
        string
            .split_whitespace()
            .take(2)
            .map(|element| element.parse::<f32>())
            .filter(|element| element.is_ok())
            .map(|element| element.unwrap())
            .collect()
    })
    .filter(|item| item.len() == 2)
    .map(|mut item| (item.swap_remove(0), item.swap_remove(0)))
    .unzip()
}
