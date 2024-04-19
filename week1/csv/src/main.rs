use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn read_csv(file_path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let file = File::open(&Path::new(file_path))?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut numbers = Vec::new();
    let headers = reader.headers()?;
    println!("Headers: {:?}", headers);

    for result in reader.records() {
        let record = result?;
        let number: f64 = record[0].parse()?;
        let word: &str = &record[1];
        println!("Number: {}, Word: {}", number, word);
        numbers.push(number);
    }

    Ok(numbers)
}

fn mean(numbers: &Vec<f64>) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

fn min(numbers: &Vec<f64>) -> f64 {
    *numbers
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn max(numbers: &Vec<f64>) -> f64 {
    *numbers
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn printout(numbers: &Vec<f64>) {
    println!("Numbers: {:?}", numbers);
    println!("Mean: {}", mean(numbers));
    println!("Min: {}", min(numbers));
    println!("Max: {}", max(numbers));
}

fn main() {
    match read_csv("numbers.csv") {
        Ok(numbers) => printout(&numbers),
        Err(err) => println!("Error: {}", err),
    }
}
