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
    let lines = read_lines(filename).expect("Could not read lines from file.");
    // Each line is also a Result, so unpack those (shadowing prev. lines)
    let lines = lines.map(|x| x.unwrap());
    // Split columns into f32 Vec
    (x_vals, y_vals) = read_xy_pairs(lines);
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

    let filename2 = "/home/rhys/Documents/rew_test/test2.dat";
    let lines2 = read_lines(filename2).expect("Could not read lines from file.");
    let lines2 = lines2.map(|x| x.unwrap());
    println!("{:?}", lines2.next());
    let new = process_rows(lines2, 3);
    let new = transpose(new);
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

fn process_rows(i: impl Iterator<Item = String>, n: u32) -> Vec<Vec<f32>> {
    let num_cols = usize::try_from(n).unwrap();

    i.map(|string| -> Vec<f32> {
        string
            .split_whitespace()
            .take(num_cols)
            .map(|element| element.parse::<f32>())
            .filter(|element| element.is_ok())
            .map(|element| element.unwrap())
            .collect()
    })
    .filter(|item| item.len() == num_cols)
    .collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
