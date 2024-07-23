use polars::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "/home/rhys/Documents/rew_test/test.dat";
    let x_vals;
    let y_vals;
    if let Ok(lines) = read_lines(filename) {
        let lines = lines.map(|x| x.unwrap());
        (x_vals, y_vals) = read_xy_pairs(lines);
    } else {
        panic!()
    }
    println!("{:?}", x_vals);
    let df: PolarsResult<DataFrame> = df!(
        "X" => x_vals,
        "Y" => y_vals,
    );
    let df = df.unwrap();
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
