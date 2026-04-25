// Part 1 - csvprof: CSV Data Profiler (reused in Part 2)
// COSC 352 Project 8 - Michael Miller

use anyhow::{Context, Result};
use clap::Parser;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Custom Error Type

#[derive(Debug)]
pub enum CsvProfError {
    Io(io::Error),
    Csv(csv::Error),
    EmptyFile,
}

impl fmt::Display for CsvProfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvProfError::Io(e) => write!(f, "IO error: {}", e),
            CsvProfError::Csv(e) => write!(f, "CSV error: {}", e),
            CsvProfError::EmptyFile => write!(f, "File is empty"),
        }
    }
}

impl std::error::Error for CsvProfError {}

impl From<io::Error> for CsvProfError {
    fn from(e: io::Error) -> Self { CsvProfError::Io(e) }
}
impl From<csv::Error> for CsvProfError {
    fn from(e: csv::Error) -> Self { CsvProfError::Csv(e) }
}

// ColumnAnalyzer Trait

pub trait ColumnAnalyzer {
    fn feed(&mut self, value: &str);
    fn report(&self, col_name: &str);
}

// Column Type

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    Integer,
    Float,
    Text,
}

// Stats Analyzer

pub struct StatsAnalyzer {
    pub count: usize,
    pub null_count: usize,
    pub col_type: ColumnType,
    pub float_vals: Vec<f64>,
    pub text_freq: HashMap<String, usize>,
    pub total_len: usize,
}

impl StatsAnalyzer {
    pub fn new() -> Self {
        StatsAnalyzer {
            count: 0,
            null_count: 0,
            col_type: ColumnType::Integer,
            float_vals: Vec::new(),
            text_freq: HashMap::new(),
            total_len: 0,
        }
    }
}

impl ColumnAnalyzer for StatsAnalyzer {
    fn feed(&mut self, value: &str) {
        self.count += 1;
        let trimmed = value.trim();
        if trimmed.is_empty() {
            self.null_count += 1;
            return;
        }
        self.total_len += trimmed.len();
        if let Ok(i) = trimmed.parse::<i64>() {
            self.float_vals.push(i as f64);
        } else if let Ok(fv) = trimmed.parse::<f64>() {
            self.col_type = ColumnType::Float;
            self.float_vals.push(fv);
        } else {
            self.col_type = ColumnType::Text;
            *self.text_freq.entry(trimmed.to_string()).or_insert(0) += 1;
        }
    }

    fn report(&self, col_name: &str) {
        println!("Column: {}", col_name);
        println!("  Type   : {:?}", self.col_type);
        println!("  Count  : {}", self.count);
        println!("  Nulls  : {}", self.null_count);
        let non_null = self.count - self.null_count;
        match self.col_type {
            ColumnType::Text => {
                println!("  Unique : {}", self.text_freq.len());
                if non_null > 0 {
                    println!("  Avg len: {:.1}", self.total_len as f64 / non_null as f64);
                }
                let mut top: Vec<_> = self.text_freq.iter().collect();
                top.sort_by(|a, b| b.1.cmp(a.1));
                for (val, cnt) in top.iter().take(3) {
                    println!("  Top    : '{}' ({})", val, cnt);
                }
            }
            _ => {
                if !self.float_vals.is_empty() {
                    let mut s = self.float_vals.clone();
                    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let n = s.len();
                    let sum: f64 = s.iter().sum();
                    let mean = sum / n as f64;
                    let median = if n % 2 == 0 { (s[n/2-1]+s[n/2])/2.0 } else { s[n/2] };
                    println!("  Min    : {}", s[0]);
                    println!("  Max    : {}", s[n-1]);
                    println!("  Mean   : {:.2}", mean);
                    println!("  Median : {}", median);
                }
            }
        }
        println!();
    }
}

// Streaming CSV Reader

pub fn stream_csv(path: &str) -> Result<Vec<(String, StatsAnalyzer)>, CsvProfError> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().from_reader(buf);
    let headers: Vec<String> = rdr.headers()?.iter().map(|h| h.to_string()).collect();
    if headers.is_empty() { return Err(CsvProfError::EmptyFile); }
    let mut analyzers: Vec<StatsAnalyzer> = headers.iter().map(|_| StatsAnalyzer::new()).collect();
    for result in rdr.records() {
        let record = result?;
        for (i, field) in record.iter().enumerate() {
            if i < analyzers.len() { analyzers[i].feed(field); }
        }
    }
    Ok(headers.into_iter().zip(analyzers).collect())
}
