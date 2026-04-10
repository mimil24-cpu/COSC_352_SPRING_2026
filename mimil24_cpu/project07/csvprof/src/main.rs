use anyhow::{Context, Result};
use clap::Parser;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser)]
#[command(name = "csvprof")]
#[command(about = "CSV Data Profiler - COSC 352 Project 7")]
struct Cli {
    /// Path to CSV file (use '-' for stdin)
    file: String,
    
    /// Show percentiles (p5/p25/p75/p95)
    #[arg(long)]
    percentiles: bool,
    
    /// Show value histogram for categorical columns
    #[arg(long)]
    histogram: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum ColumnType {
    Integer,
    Float,
    Boolean,
    Date,
    Categorical,
    Text,
}

trait Profiler {
    fn update(&mut self, value: &str);
    fn infer_type(&mut self);
    fn report(&self, name: &str, show_percentiles: bool, show_histogram: bool) -> String;
}

struct ColumnProfiler {
    col_type: ColumnType,
    total: usize,
    nulls: usize,
    values: HashMap<String, usize>,
    nums: Vec<f64>,
    lengths: Vec<usize>,
}

impl ColumnProfiler {
    fn new() -> Self {
        Self {
            col_type: ColumnType::Text,
            total: 0,
            nulls: 0,
            values: HashMap::new(),
            nums: Vec::new(),
            lengths: Vec::new(),
        }
    }

    fn percentile(sorted: &[f64], p: f64) -> f64 {
        if sorted.is_empty() {
            return 0.0;
        }
        let idx = ((p / 100.0) * (sorted.len() - 1) as f64) as usize;
        sorted[idx]
    }
}

impl Profiler for ColumnProfiler {
    fn update(&mut self, value: &str) {
        self.total += 1;
        let trimmed = value.trim();
        
        if trimmed.is_empty() {
            self.nulls += 1;
            return;
        }

        *self.values.entry(trimmed.to_string()).or_insert(0) += 1;
        self.lengths.push(trimmed.len());

        if let Ok(n) = trimmed.parse::<i64>() {
            self.nums.push(n as f64);
        } else if let Ok(n) = trimmed.parse::<f64>() {
            self.nums.push(n);
        }
    }

    fn infer_type(&mut self) {
        let non_null = self.total - self.nulls;
        if non_null == 0 {
            return;
        }

        // Check for boolean
        let bool_vals: std::collections::HashSet<&str> =
            ["true", "false", "yes", "no", "1", "0", "t", "f"]
                .iter()
                .cloned()
                .collect();
        
        let all_bool = self.values.keys().all(|k| bool_vals.contains(k.to_lowercase().as_str()));
        
        if all_bool && self.values.len() <= 2 {
            self.col_type = ColumnType::Boolean;
            return;
        }

        // Check for numeric
        if self.nums.len() == non_null && !self.nums.is_empty() {
            let all_int = self.nums.iter().all(|n| n.fract() == 0.0);
            self.col_type = if all_int {
                ColumnType::Integer
            } else {
                ColumnType::Float
            };
            return;
        }

        // Check for date-like strings
        let date_like = self
            .values
            .keys()
            .filter(|v| {
                let s = v.trim();
                s.len() >= 6 && (s.contains('-') || s.contains('/'))
            })
            .count();

        if date_like == non_null {
            self.col_type = ColumnType::Date;
            return;
        }

        // Categorical vs Text based on cardinality
        if self.values.len() <= 20 {
            self.col_type = ColumnType::Categorical;
        } else {
            self.col_type = ColumnType::Text;
        }
    }

    fn report(&self, name: &str, show_percentiles: bool, show_histogram: bool) -> String {
        let mut s = String::new();
        let null_pct = if self.total > 0 {
            (self.nulls as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        s.push_str(&format!("\n{}\n", "=".repeat(60)));
        s.push_str(&format!("Column: {}\n", name));
        s.push_str(&format!("{}\n", "-".repeat(60)));
        s.push_str(&format!("Type       : {:?}\n", self.col_type));
        s.push_str(&format!("Rows       : {}\n", self.total));
        s.push_str(&format!("Nulls      : {} ({:.1}%)\n", self.nulls, null_pct));
        s.push_str(&format!("Unique     : {}\n", self.values.len()));

        // Warning for constant columns
        if self.values.len() == 1 && self.total > 1 {
            s.push_str("⚠️  WARNING: Constant column (all values identical)\n");
        }

        // Numeric statistics
        if matches!(self.col_type, ColumnType::Integer | ColumnType::Float)
            && !self.nums.is_empty()
        {
            let mut sorted = self.nums.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let min = sorted[0];
            let max = *sorted.last().unwrap();
            let mean = self.nums.iter().sum::<f64>() / self.nums.len() as f64;
            let median = Self::percentile(&sorted, 50.0);
            
            let variance = self.nums.iter()
                .map(|n| (n - mean).powi(2))
                .sum::<f64>() / self.nums.len() as f64;
            let std_dev = variance.sqrt();

            s.push_str(&format!("Min        : {}\n", min));
            s.push_str(&format!("Max        : {}\n", max));
            s.push_str(&format!("Mean       : {:.4}\n", mean));
            s.push_str(&format!("Median     : {:.4}\n", median));
            s.push_str(&format!("Std Dev    : {:.4}\n", std_dev));

            if show_percentiles {
                s.push_str(&format!(
                    "Percentiles: p5={:.2} p25={:.2} p75={:.2} p95={:.2}\n",
                    Self::percentile(&sorted, 5.0),
                    Self::percentile(&sorted, 25.0),
                    Self::percentile(&sorted, 75.0),
                    Self::percentile(&sorted, 95.0)
                ));
            }
        }

        // String length stats
        if !self.lengths.is_empty() {
            let min_len = *self.lengths.iter().min().unwrap();
            let max_len = *self.lengths.iter().max().unwrap();
            s.push_str(&format!("Str Length : min={} max={}\n", min_len, max_len));
        }

        // Top 5 values
        let mut freq: Vec<_> = self.values.iter().collect();
        freq.sort_by(|a, b| b.1.cmp(a.1));
        
        s.push_str("\nTop 5 Values:\n");
        for (val, count) in freq.iter().take(5) {
            let display_val = if val.len() > 30 {
                format!("{}...", &val[..27])
            } else {
                val.to_string()
            };
            s.push_str(&format!("  {:32} : {}\n", display_val, count));
        }

        // Histogram for categorical/boolean
        if show_histogram
            && matches!(self.col_type, ColumnType::Categorical | ColumnType::Boolean)
        {
            s.push_str("\nValue Histogram:\n");
            let max_count = freq.iter().map(|(_, c)| **c).max().unwrap_or(1);
            
            for (val, count) in freq.iter().take(10) {
                let bar_len = ((**count as f64 / max_count as f64) * 40.0) as usize;
                let bar_len = bar_len.max(1);
                let display_val = if val.len() > 20 {
                    format!("{}...", &val[..17])
                } else {
                    val.to_string()
                };
                s.push_str(&format!(
                    "  {:20} |{:<40}| {}\n",
                    display_val,
                    "█".repeat(bar_len),
                    count
                ));
            }
        }

        s
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let reader: Box<dyn BufRead> = if cli.file == "-" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(
            File::open(&cli.file).with_context(|| format!("Cannot open file: {}", cli.file))?,
        ))
    };

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(reader);

    let headers = csv_reader.headers()?.clone();
    let mut profilers: Vec<ColumnProfiler> = headers.iter().map(|_| ColumnProfiler::new()).collect();

    // Stream through rows (memory-efficient)
    for result in csv_reader.records() {
        let record = result?;
        for (i, field) in record.iter().enumerate() {
            if let Some(profiler) = profilers.get_mut(i) {
                profiler.update(field);
            }
        }
    }

    // Infer types after seeing all data
    for profiler in profilers.iter_mut() {
        profiler.infer_type();
    }

    // Generate report
    println!("\n{}", "═".repeat(60));
    println!("  CSV PROFILER REPORT");
    println!("  File: {}", cli.file);
    println!("{}", "═".repeat(60));

    for (i, header) in headers.iter().enumerate() {
        if let Some(profiler) = profilers.get(i) {
            print!("{}", profiler.report(header, cli.percentiles, cli.histogram));
        }
    }

    println!("\n{}\n", "═".repeat(60));
    Ok(())
}