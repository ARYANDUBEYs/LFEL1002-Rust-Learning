use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error; 

#[derive(Debug)]
struct Record {
    name: String,
    age: u32,
    score: f64,
} 

impl Record {
    
    fn from_csv_line(line: &str) -> Result<Record, Box<dyn Error>> {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 3 {
            return Err("Invalid CSV line".into());
        } 

        let name = fields[0].to_string();
        let age = fields[1].parse::<u32>()?;
        let score = fields[2].parse::<f64>()?; 

        Ok(Record { name, age, score })
    }
} 

fn main() -> Result<(), Box<dyn Error>> {
    let filename= "data.csv";
    let mut records: Vec<Record> = Vec::new(); 

    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let record = Record::from_csv_line(&line)?;
        records.push(record);
    } 

    let total_score: f64 = records.iter().map(|r| r.score).sum();
    let average_score = total_score / records.len() as f64; 

    println!("Number of records: {}", records.len());
    println!("Average score: {:.2}", average_score); 

    Ok(())
}