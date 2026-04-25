// Part 2 - Correlate Baltimore City Open Data
// Reuses stream_csv, StatsAnalyzer, ColumnAnalyzer from lib.rs (Part 1)
// Research Question: Do Baltimore neighborhoods with more 311 service requests
// also have higher crime counts?

use csvprof::stream_csv;
use anyhow::Result;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

const CRIME_CSV: &str = "data/bpd_crime.csv";
const CALLS_CSV: &str = "data/311_calls.csv";

fn col_index(path: &str, col: &str) -> Result<usize> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(file));
    let headers = rdr.headers()?.clone();
    for (i, h) in headers.iter().enumerate() {
        if h.trim().to_lowercase() == col.to_lowercase() { return Ok(i); }
    }
    anyhow::bail!("Column {} not found in {}", col, path);
}

fn count_by(path: &str, col: &str) -> Result<HashMap<String, usize>> {
    let idx = col_index(path, col)?;
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(file));
    let mut map: HashMap<String, usize> = HashMap::new();
    for result in rdr.records() {
        let rec = result?;
        if let Some(v) = rec.get(idx) {
            let k = v.trim().to_string();
            if !k.is_empty() { *map.entry(k).or_insert(0) += 1; }
        }
    }
    Ok(map)
}

fn main() -> Result<()> {
    println!("=== Part 2: Baltimore City Open Data Correlation ===");
    println!("Research Question: Do neighborhoods with more 311 service requests");
    println!("also have higher crime counts?
");

    // Use Part 1 stream_csv to profile both files
    println!("[Profiling {} using Part 1 stream_csv]", CRIME_CSV);
    let crime_prof = stream_csv(CRIME_CSV).expect("Could not read crime CSV");
    println!("  {} columns, {} rows", crime_prof.len(),
        crime_prof.first().map(|(_,a)| a.count).unwrap_or(0));

    println!("[Profiling {} using Part 1 stream_csv]", CALLS_CSV);
    let calls_prof = stream_csv(CALLS_CSV).expect("Could not read 311 CSV");
    println!("  {} columns, {} rows
", calls_prof.len(),
        calls_prof.first().map(|(_,a)| a.count).unwrap_or(0));

    // Count crimes per neighborhood
    let crime_map = count_by(CRIME_CSV, "Neighborhood")?;
    // Count 311 calls per neighborhood
    let calls_map = count_by(CALLS_CSV, "Neighborhood")?;

    // Inner join on neighborhood name
    let mut joined: Vec<(String, usize, usize)> = Vec::new();
    for (hood, crimes) in &crime_map {
        if let Some(calls) = calls_map.get(hood) {
            joined.push((hood.clone(), *crimes, *calls));
        }
    }

    if joined.is_empty() {
        println!("No overlapping neighborhoods found between the two datasets.");
        return Ok(());
    }

    joined.sort_by(|a, b| b.2.cmp(&a.2));

    println!("=== Top 20 Neighborhoods by 311 Call Volume ===");
    println!("{:<35} {:>10} {:>12}", "Neighborhood", "311 Calls", "Crimes");
    println!("{}", "-".repeat(60));
    for (h, cr, ca) in joined.iter().take(20) {
        println!("{:<35} {:>10} {:>12}", h, ca, cr);
    }

    // Pearson correlation: x = 311 calls, y = crime count
    let n = joined.len() as f64;
    let mx: f64 = joined.iter().map(|(_,_,c)| *c as f64).sum::<f64>() / n;
    let my: f64 = joined.iter().map(|(_,cr,_)| *cr as f64).sum::<f64>() / n;
    let num: f64 = joined.iter().map(|(_,cr,c)| (*c as f64-mx)*(*cr as f64-my)).sum();
    let dx: f64 = joined.iter().map(|(_,_,c)| (*c as f64-mx).powi(2)).sum::<f64>().sqrt();
    let dy: f64 = joined.iter().map(|(_,cr,_)| (*cr as f64-my).powi(2)).sum::<f64>().sqrt();
    let r = if dx*dy == 0.0 { 0.0 } else { num/(dx*dy) };

    println!("
=== Correlation Results ===");
    println!("Matched neighborhoods : {}", joined.len());
    println!("Mean 311 calls        : {:.1}", mx);
    println!("Mean crime count      : {:.1}", my);
    println!("Pearson r             : {:.4}", r);
    let msg = if r > 0.5 { "Strong positive" } else if r > 0.2 { "Moderate positive" } else if r > 0.0 { "Weak positive" } else { "No/negative" };
    println!("Result: {} correlation between 311 calls and crime count per neighborhood.", msg);
    Ok(())
}
