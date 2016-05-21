extern crate linear_regression;
extern crate csv;

use std::io;
use std::io::prelude::*;

fn get_mileage () -> i32 {
    let mut parsing;
    loop {
        println!("Please enter a car mileage");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");
        parsing = input.trim().parse::<i32>();
        match parsing {
            Ok(_) => break,
            Err(_) => println!("mileage must be a valid number"),
        };
    };
    parsing.unwrap()
}

fn main () {
    let row: Vec<(f64, f64, f64, f64)> = match std::fs::metadata("Theta.csv") {
        Ok(_) => {
            let mut rdr = csv::Reader::from_file("Theta.csv").unwrap().has_headers(false);
            rdr.decode().collect::<csv::Result<Vec<(f64, f64, f64, f64)>>>().unwrap()
        },
        Err(_) => vec![(0.0, 0.0, 0.0, 0.0)],
    };
    let mileage = match row[0] {
        (0.0, 0.0, _, _) => get_mileage() as f64,
        _ => (get_mileage() as f64 - row[0].3) / (row[0].2 - row[0].3),
    };
    let estimation = row[0].0 + (row[0].1 * mileage);

    println!("Estimation: {}", estimation)
}
