
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'')
        .trim(csv::Trim::All)
        .from_reader(file);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "/home/rhys/Documents/rew_test/test.dat";
    read_csv(filename)
}
// use ndarray::Array2;
// use ndarray_csv::Array2Reader;
// use std::error::Error;
// use std::fs::File;

// fn read_csv(path_to_file: &str) -> Result<Array2<u64>, Box<dyn Error>> {
// let file = File::open(path_to_file)?;
// let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
// Ok(reader.deserialize((2, 3))?)
// }

// fn main() {
// let array = read_csv("/home/rhys/Documents/rew_test/test.dat");
// }

// use std::fs::File;
use std::io::{self, BufRead};

//if let Ok(lines) = read_lines("/home/rhys/Documents/rew_test/a2b1+A769_R1_1d_d1_FES") {
// Consumes the iterator, returns an (Optional) String
// for line in lines.flatten() {
// let ll = line.as_str().split_whitespace();
// let mut ll = line.split_whitespace();
// c1_vec.push(ll.next().unwrap());
//    c2_vec.push(ll.next().unwrap());
//}
//}

// fn main() {
// let data = "
// A       1    Pass
// B   2         Fail
// C   3       Faili
// ";
// let file = File::open(
// let data = io::BufReader::new(file);
//
// for line in data.lines() {
// let line = line.trim();
// if line.is_empty() {
// continue;
// }
// let mut parts = line.split_whitespace();
// let tuple = (
// parts.next().unwrap(),
// parts.next().unwrap().parse::<f32>().unwrap(),
// parts.next().unwrap(),
// );
// println!("{:?}", tuple);
// }
// }

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_xy_pairs(i: impl Iterator<Item = String>) -> (Vec<f32>, Vec<f32>) {
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
